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
    #[clap(short = 'i', long)]
    infile: String,

    // Path to the output CSV file
    #[clap(short = 'o', long)]
    outfile: String,


    /// Whether to output the adherence at drug level, or for overall patient
    #[clap(short, long)] 
    druglevel: bool,
}

fn main() {
    // example CLI invokation while developing
    // cargo run -- --infile=C:\Users\windo\Documents\rust\propdayscov-rs\test\files\one_patient_one_drug.csv --outfile=C:\Users\windo\Documents\showme.csv --druglevel
    let args = Args::parse();

    // user must indicate csv with claims information
    // single file for now. multiple files in the future?  possibly also eligibility files?
    let mut patients = patient_parsing::parse_doses(args.infile).unwrap();

    // calculate PDC, including all calendar shifting needed.  use Rayon magic for parallelization
    patients.par_iter_mut().for_each(|(_id, patient)| {
        patient.calculate_pdc();
    });

    // write results out to file
    match patient_parsing::export_results(&args.outfile, patients, args.druglevel) {
        Err(e) => panic!("Error when writing results -- {:?}", e),
        Ok(()) => {println!("PDC successfully calculated, results written to {:?}", &args.outfile)}
    }

    // TODO
    // 1. add more tests.  multiple patients, multiple drugs, etc
    // 2. add support for multiple imported CSVs
        // as comma-separated list of cli filepaths? ugh...
        // maybe as directory?
    // 3. add data checks on imports
        // check for duplicate doses
        // check for ..... ?
}
