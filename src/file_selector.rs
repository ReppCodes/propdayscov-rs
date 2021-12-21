use nfd2::{Response};
use std::error::Error;
use std::path::PathBuf;

pub fn select_file() -> Result<PathBuf, Box<dyn Error>> {
    match nfd2::open_file_dialog(None, None).expect("Error opening file dialog.") {
        Response::Okay(file_path) =>  Ok(file_path),
        Response::Cancel => println!("User canceled"),
    }
}
