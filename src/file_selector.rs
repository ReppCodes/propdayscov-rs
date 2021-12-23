use nfd2::{Response};
use std::error::Error;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug)]
pub struct DialogCancelError {
    details: String
}

impl DialogCancelError {
    fn new(msg: &str) -> DialogCancelError {
        DialogCancelError{details: msg.to_string()}
    }
}

impl fmt::Display for DialogCancelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for DialogCancelError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn select_file() -> Result<PathBuf, DialogCancelError> {
    match nfd2::open_file_dialog(None, None).expect("Error opening input file dialog.") {
        Response::Cancel => Err(DialogCancelError::new("User cancelled file selection.")),
        // handle multiple files in future, get this working with single file first
        Response::OkayMultiple(_files) => Err(DialogCancelError::new("Please select a single file.")),
        Response::Okay(file_path) =>  Ok(file_path),
        
    }
}
