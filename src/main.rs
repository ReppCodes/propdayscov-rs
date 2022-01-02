use std::collections::HashMap;
use dose_parser::Patient;
use rayon::prelude::*;

mod dose_parser;
mod file_selector;
mod pdc;

fn main() {
    // user must select csv with claims information
    // single file for now. multiple files in the future?  possibly also eligibility files?
    let file_in = file_selector::select_file().unwrap();
    let mut patients = dose_parser::parse_doses(file_in).unwrap();

    // calculate PDC, including all calendar shifting needed.  use Rayon magic for parallelization
    let processed_patients: HashMap<String, Patient> =  patients.par_iter_mut()
        .map(|(id, patient)|{
            patient.calculate_pdc();
            (id.clone(), patient.clone())
            }
        )
        .collect();
    
    // print results to stdout for now
    for (_id, patient) in processed_patients{
        println!("{:?}", patient.overall_adherence);
        println!("{:?}", patient.drug_lvl_adherence);
    }


    // TODO
    // 1. add more tests.  multiple patients, multiple drugs, etc
    // 2. figure out how to dump this out to a file.  simple as second nfd selector and serialize out?
        // aggravating.  maybe switch to CLI arguments instead?
    // 3. add support for multiple imported CSVs
        // as comma-separated list of cli filepaths? ugh...
        // maybe as directory?
    // 4. add data checks on imports
        // check for duplicate doses
        // check for ..... ?
}
