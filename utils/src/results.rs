use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

// pub mod results {

pub type Result<T> = std::result::Result<T, AOCError>;

#[derive(Debug, PartialEq)]
pub struct AOCError(String);

impl AOCError {
    pub fn new_from_ref(message: &str) -> AOCError {
        AOCError(message.to_owned())
    }

    pub fn new(message: String) -> AOCError {
        AOCError(message)
    }
}

impl From<&'static str> for AOCError {
    fn from(message: &'static str) -> AOCError {
        AOCError::new(message.to_owned())
    }
}

impl From<std::io::Error> for AOCError {
    fn from(error: std::io::Error) -> AOCError {
        AOCError::new(error.to_string())
    }
}

impl From<std::num::ParseIntError> for AOCError {
    fn from(error: std::num::ParseIntError) -> AOCError {
        AOCError::new(error.to_string())
    }
}

impl Display for AOCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for AOCError {}
// }
