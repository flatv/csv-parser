use std::io;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum ParseError {
    IoError(io::Error),
    InvalidData(String),
    FileNotFound(String),
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound => ParseError::FileNotFound(error.to_string()),
            _ => ParseError::IoError(error)
        }
    }
}