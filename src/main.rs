mod date_shifter;
mod dose_parser;
mod file_selector;
mod pdc;

fn main() {
    // user must select csv with claims information
    // single file for now. multiple files in the future?  possibly also eligibility files?
    let file_in = file_selector::select_file().unwrap();
    let patients = dose_parser::parse_doses(file_in).unwrap();

    for (_id, mut patient) in patients {
        patient.create_calendar();
        patient.calculate_pdc();
        println!("{:?}", patient.overall_adherence);
        println!("{:?}", patient.drug_lvl_adherence);
    }

    // TODO
    // 1. add more tests.  multiple patients, multiple drugs, etc
    // 2. add multi-threading.  see TODOs scattered around
    // 3. figure out how to dump this out to a file.  simple as second nfd selector and serialize out?
    // 4. add support for multiple imported CSVs
    // 5. add data checks on imports
        // check for duplicate doses
        // check for ..... ?
}
