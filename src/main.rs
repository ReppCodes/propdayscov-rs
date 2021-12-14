use std::io;

mod dose_parser;
mod date_shifter;
mod pdc;

fn main() {
    // entry point into the library

    // bring in doses library, capture directly into parser
    let file_in = io::stdin();
    let doses = dose_parser::parse_doses(file_in);
    for entry in doses{
        // show that we parsed correctly while developing
        // TODO remove this when further along
        println!("{:?}", entry);
    }

    // TODO -- see how to add error handling to the dose parser
    // if let Err(err) = example() {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }

    // Perform date shifting

    // calculate PDC on shifted dates

    // print results to stdout
}
