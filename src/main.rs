use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
struct Dose {
    patient_id: String,
    drug_name: String,
    days_supply: u8,
    fill_date: String,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for dose in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let dose: Dose = dose?;
        println!("{:?}", dose);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
