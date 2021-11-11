use std::error::Error;
use std::fmt::{Display, Formatter, self};
use std::io;
use std::num;
use reqwest;

#[derive(Debug)]
pub enum AocError {
    Input(io::Error),
    BadInt(num::ParseIntError),
    BadFloat(num::ParseFloatError),
    BadRequest(reqwest::Error),
    TooEarly,
    Misc(String)
}

impl Display for AocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AocError::Input(error) => write!(f, "Error opening file: {}", error),
            AocError::BadInt(error) => write!(f, "Bad integer: {}", error),
            AocError::BadFloat(error) => write!(f, "Bad float: {}", error),
            AocError::BadRequest(error) => write!(f, "Bad request: {}", error),
            AocError::TooEarly => write!(f, "Can't start this puzzle, it hasn't unlocked yet"),
            AocError::Misc(message) => write!(f, "Error running problem: {}", message)
        }
    }
}

impl Error for AocError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AocError::Input(error) => Some(error),
            AocError::BadInt(error) => Some(error),
            AocError::BadFloat(error) => Some(error),
            AocError::BadRequest(error) => Some(error),
            _ => None
        }
    }
}

impl From<io::Error> for AocError {
    fn from(error: io::Error) -> Self {
        AocError::Input(error)
    }
}

impl From<num::ParseIntError> for AocError {
    fn from(error: num::ParseIntError) -> Self {
        AocError::BadInt(error)
    }
}

impl From<num::ParseFloatError> for AocError {
    fn from(error: num::ParseFloatError) -> Self {
        AocError::BadFloat(error)
    }
}

impl From<reqwest::Error> for AocError {
    fn from(error: reqwest::Error) -> Self {
        AocError::BadRequest(error)
    }
}
