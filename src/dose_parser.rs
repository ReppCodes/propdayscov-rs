use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug)]
pub struct PatientCalendar {
    drug_name: String,
    coverage_date: NaiveDate,
    covered_bool: bool
}

impl PatientCalendar{
    fn new() -> PatientCalendar {
        PatientCalendar {
            drug_name: String,
            coverage_date: NaiveDate,
            covered_bool: bool
        }
    }
}

#[derive(Debug)]
pub struct Patient {
    pub patient_id: String,
    pub adherence: f64,
    pub drug_list: Vec<String>,
    pub given_doses: Vec<Dose>,
    pub shifted_calendar: PatientCalendar
}

impl Patient {
    fn new(patient_id: String) -> Patient {
        Patient{
            patient_id: patient_id,
            adherence: 0.0,
            drug_list: Vec::new(),
            given_doses: Vec::new(),
            shifted_calendar: PatientCalendar::new()
        }
    }
    fn create_calendar(&self) -> () {

        self.shifted_calendar;
    }
}

pub fn create_calendar(patient: Patient) -> Result<PatientCalendar, Box<dyn Error>> {
    let dummy_date: NaiveDate = NaiveDate::from_ymd(1999, 01, 01);
    let patient_cal: PatientCalendar = PatientCalendar { patient_id: "johnsmith".to_string(), drug_name: "drugname".to_string(), coverage_date:  dummy_date};

    Ok(patient_cal)
}

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize, Clone)]
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

pub fn parse_doses(file_in: PathBuf) -> Result<HashMap<String, Patient>, Box<dyn Error>> {
    let csv_reader = csv::Reader::from_path(file_in);
    let mut patient_map: HashMap<String, Patient> = HashMap::new();
    for record in csv_reader.unwrap().deserialize() {
        let dose: Dose = record?;
        let patient_id: String = dose.patient_id.clone();

        if !patient_map.contains_key(&patient_id){
            let mut new_pat: Patient = Patient::new(patient_id.clone());
            new_pat.given_doses.push(dose);
            patient_map.insert(patient_id, new_pat);
        }
        else {
            (*patient_map.get_mut(&dose.patient_id).unwrap()).given_doses.push(dose);
        }
        
    }
    Ok(patient_map)
}

