use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;

use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Patient {
    pub patient_id: String,
    pub overall_adherence: f64,
    pub drug_lvl_adherence: HashMap<String, f64>,
    #[serde(skip_serializing)]
    pub drug_list: HashSet<String>,
    #[serde(skip_serializing)]
    pub given_doses: HashMap<String, Vec<Dose>>,
}

impl Patient {
    fn new(patient_id: String) -> Patient {
        Patient {
            patient_id: patient_id,
            overall_adherence: 0.0,
            drug_lvl_adherence: HashMap::new(),
            drug_list: HashSet::new(),
            given_doses: HashMap::new(),
        }
    }
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
    use chrono::NaiveDate;
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
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub fn parse_doses(file_in: PathBuf) -> Result<HashMap<String, Patient>, Box<dyn Error>> {
    let csv_reader = csv::Reader::from_path(file_in);
    let mut patient_map: HashMap<String, Patient> = HashMap::new();
    for record in csv_reader
        .expect("Please select a single, correctly formatted, CSV file.")
        .deserialize()
    {
        let dose: Dose = record?;
        let patient_id: String = dose.patient_id.clone();

        if !patient_map.contains_key(&dose.patient_id) {
            // New patient
            let mut new_pat: Patient = Patient::new(dose.patient_id.clone());
            new_pat
                .given_doses
                .insert(dose.drug_name.clone(), Vec::new());
            new_pat.drug_list.insert(dose.drug_name.clone());
            new_pat
                .given_doses
                .get_mut(&dose.drug_name)
                .unwrap()
                .push(dose);
            patient_map.insert(patient_id, new_pat);
        } else if !patient_map
            .get(&dose.patient_id)
            .unwrap()
            .given_doses
            .contains_key(&dose.drug_name)
        {
            // Patient already exist, but new drug found
            // Add drug to patient
            (*patient_map.get_mut(&dose.patient_id).unwrap())
                .given_doses
                .insert(dose.drug_name.clone(), Vec::new());
            (*patient_map.get_mut(&dose.patient_id).unwrap())
                .drug_list
                .insert(dose.drug_name.clone());

            // Add dose to drug
            (*patient_map.get_mut(&dose.patient_id).unwrap())
                .given_doses
                .get_mut(&dose.drug_name)
                .unwrap()
                .push(dose);
        } else {
            // Patient already exists and has drug.  Add dose.
            (*patient_map.get_mut(&dose.patient_id).unwrap())
                .given_doses
                .get_mut(&dose.drug_name)
                .unwrap()
                .push(dose);
        }
    }
    Ok(patient_map)
}

// Core program logic moved down here away from structs and ser/deser for clarity
impl Patient {
    pub fn calculate_pdc(&mut self) -> () {
        // calculate shifted calendar
        let mut shifted_calendar: HashMap<String, HashMap<NaiveDate, bool>> = HashMap::new();
        for (drug_name, dose_list) in &mut self.given_doses {
            // sort given_doses by date
            dose_list.sort_by(|a, b| a.fill_date.cmp(&b.fill_date));

            // iterate through doses, generating shifted start and stop dates
            let mut first_start_dt = NaiveDate::from_ymd(2900, 1, 1);
            let mut last_end_dt = NaiveDate::from_ymd(1900, 1, 1);
            let mut prior_end_dt = NaiveDate::from_ymd(1900, 1, 2);
            let mut covered_dates: HashSet<NaiveDate> = HashSet::new();
            for dose in dose_list {
                let mut start_dt: NaiveDate = dose.fill_date;

                // shift start date to handle early refills
                if start_dt < prior_end_dt {
                    start_dt = prior_end_dt + Duration::days(1);
                }

                // calculate end date off of shifted start date
                // TODO check whether we need a -1 here, to account for pill taken on day of fill
                let end_dt: NaiveDate = start_dt + Duration::days(dose.days_supply.into()) - Duration::days(1);
                prior_end_dt = end_dt;

                // generate set of all covered days for this dose, pushed into set for given drug
                let mut dt = start_dt;
                while dt <= end_dt {
                    covered_dates.insert(dt);
                    dt = dt + Duration::days(1);
                }

                // bookkeeping on start/end dates for patient analysis window
                // TODO should these be passed in by user, optionally?
                // this doesn't allow for eligiblity if it's available, but tbh it never is....
                if end_dt > last_end_dt {
                    last_end_dt = end_dt;
                }
                if start_dt < first_start_dt {
                    first_start_dt = start_dt;
                }
            }

            // generate full range of days for patient analysis window
            let mut all_dates: HashSet<NaiveDate> = HashSet::new();
            let mut dt = first_start_dt;
            while dt <= last_end_dt {
                all_dates.insert(dt);
                dt = dt + Duration::days(1);
            }

            let mut coverage_cal: HashMap<NaiveDate, bool> = HashMap::new();
            for date in all_dates {
                if covered_dates.contains(&date) {
                    coverage_cal.insert(date, true);
                } else {
                    coverage_cal.insert(date, false);
                }
            }
            shifted_calendar.insert(drug_name.clone(), coverage_cal);
        }

        // calculate PDC
        let mut overall_total_days: HashSet<NaiveDate> = HashSet::new();
        let mut overall_covered_days: HashSet<NaiveDate> = HashSet::new();
        for (drug_name, calendar) in shifted_calendar {
            let mut numerator = 0;
            let mut denominator = 0;
            for (date, covered) in calendar {
                if covered == true {
                    numerator += 1;
                    overall_covered_days.insert(date);
                }
                denominator += 1;
                overall_total_days.insert(date);
                let drug_adh: f64 = numerator as f64 / denominator as f64;
                self.drug_lvl_adherence.insert(drug_name.clone(), drug_adh);
            }
        }
        self.overall_adherence =
            overall_covered_days.len() as f64 / overall_total_days.len() as f64;
    }
}
