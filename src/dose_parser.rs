use std::error::Error;
use std::path::PathBuf;

use chrono::NaiveDate;
use serde::Deserialize;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Dose {
    patient_id: String,
    drug_name: String,
    days_supply: u16,
    #[serde(with = "mmddyyyy_fmt")]
    fill_date: NaiveDate,
}

#[allow(dead_code)]
mod mmddyyyy_fmt {
    // format to bring string-based dates in and out of the Dose struct
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%m/%d/%Y";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub fn parse_doses(file_in: PathBuf) -> Result<Vec<Dose>, Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_reader(file_in);
    let mut doses: Vec<Dose> = Vec::new();
    for record in csv_reader.deserialize() {
        let dose: Dose = record?;
        doses.push(dose);
    }
    Ok(doses)
}
