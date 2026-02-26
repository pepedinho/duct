use std::fmt;

#[derive(Debug)]
pub struct InvalidMethodError;

impl fmt::Display for InvalidMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid HTTP method. Supported: GET, POST, DELETE")
    }
}
