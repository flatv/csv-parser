use std::fmt;
use std::fmt::Formatter;
use std::io::Error;
use std::str::FromStr;

pub trait StringWriter {
    fn write(&self, value: String) -> Result<String, Error>;
}

#[derive(Eq, Hash, PartialEq)]
pub enum WriteType {
    File,
    Stdout,
}

impl fmt::Display for WriteType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            WriteType::File => write!(f, "file"),
            WriteType::Stdout => write!(f, "stdout")
        }
    }
}

impl FromStr for WriteType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sql" => Ok(WriteType::File),
            "stdout" => Ok(WriteType::Stdout),
            _ => Err("Invalid write type"),
        }
    }
}

