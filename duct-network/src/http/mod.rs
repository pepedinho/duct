use std::{fmt, str::FromStr};

pub enum Method {
    GET,
    POST,
    DELETE,
}

#[derive(Debug)]
pub struct InvalidMethodError;

impl fmt::Display for InvalidMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid HTTP method. Supported: GET, POST, DELETE")
    }
}

impl FromStr for Method {
    type Err = InvalidMethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            _ => Err(InvalidMethodError),
        }
    }
}
