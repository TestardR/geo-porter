use std::{error::Error, fmt};

#[derive(Debug)]
pub struct DomainError {
    pub message: String,
    pub error: Option<Box<dyn Error>>,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl DomainError {
    pub fn get_error_message(&self) -> String {
        String::from(&self.message)
    }
}