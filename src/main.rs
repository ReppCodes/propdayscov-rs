use clap::Parser;
use rayon::prelude::*;

mod file_selector;
mod patient_parsing;
mod pdc;

// TODO -- flesh this out.  want input file, output file, drug-level
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    // Path to the input CSV file
    #[clap(short, long)]
    infile: String,

    // Path to the output CSV file
    #[clap(short, long)]
    outfile: String,


    /// Whether to output the adherence at drug level, or for overall patient
    #[clap(short, long)] 
    druglevel: bool,
}

fn main() {
    // TODO, reinstate when ready
    // let args = Args::parse();
    // user must select csv with claims information
    // single file for now. multiple files in the future?  possibly also eligibility files?
    let file_in = file_selector::select_file().unwrap();
    let mut patients = patient_parsing::parse_doses(file_in).unwrap();

    // calculate PDC, including all calendar shifting needed.  use Rayon magic for parallelization
    patients.par_iter_mut().for_each(|(_id, patient)| {
        patient.calculate_pdc();
    });

    // print results to stdout for now
    for (_id, patient) in patients {
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
