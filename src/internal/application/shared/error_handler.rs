use crate::internal::domain::error::DomainError;
use std::error::Error;

pub struct ErrorHandler {}

impl ErrorHandler {
    pub fn application_error(error_message: &str, error: Option<Box<dyn Error>>) -> DomainError {
        DomainError {
            message: String::from(error_message),
            error,
        }
    }
}