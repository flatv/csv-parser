use crate::config::config::ProcessConfig;
use crate::reader::csv_reader::CsvRow;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, clap::ValueEnum, clap::Parser, Clone)]
pub enum ProcessingStrategy {
    Sql,
}

impl ProcessingStrategy {
    pub fn as_str(&self) -> &str {
        match self {
            ProcessingStrategy::Sql => "sql",
        }
    }
}

impl FromStr for ProcessingStrategy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sql" => Ok(ProcessingStrategy::Sql),
            _ => Err("Invalid processing strategy"),
        }
    }
}

pub trait Processor{
    fn process(&self, config: Box<dyn ProcessConfig>, rows: &Vec<CsvRow>) -> String;
}