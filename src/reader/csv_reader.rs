use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};
use crate::reader::error::ParseError;

#[derive(Debug, Clone)]
pub struct Headers {
    pub names: Vec<String>,
}

impl Headers {
    pub fn new(names: Vec<String>) -> Self {
        Self { names }
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }

    pub fn get_name(&self, index: usize) -> Option<&String> {
        self.names.get(index)
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }
}

#[derive(Debug)]
pub struct CsvRow {
    pub columns: HashMap<String, String>,
    pub headers: Headers,
}

impl CsvRow {
    pub fn new(headers: &Headers, values: &[String]) -> Result<Self, String> {
        if headers.len() != values.len() {
            return Err("Headers and values have different lengths".to_string());
        }

        let mut columns = HashMap::new();

        for (index, value) in values.iter().enumerate() {
            if let Some(header) = headers.get_name(index) {
                columns.insert(header.clone(), value.clone());
            }
        }

        Ok(Self {
            columns,
            headers: headers.clone(),
        })
    }
}

pub struct CsvParser;

impl CsvParser {
    pub fn parse_file(file_path: &str, separator: char) -> Result<Vec<CsvRow>, ParseError> {
        let file = File::open(file_path).map_err(ParseError::from)?;
        let reader = BufReader::new(file);

        let mut lines = reader.lines();

        let headers = match lines.next() {
            Some(Ok(line)) => Headers::new(parse_csv_line(&line, separator)),
            Some(Err(e)) => return Err(e.into()),
            None => return Err(ParseError::InvalidData("Empty file".to_string())),
        };

        let mut rows = Vec::new();

        for line in lines {
            let line = line?;
            let values: Vec<String> = parse_csv_line(&line, separator);

            let row = CsvRow::new(&headers, &values)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

            rows.push(row);
        }

        Ok(rows)
    }
}

fn parse_csv_line(line: &str, separator: char) -> Vec<String> {
    line.split(separator)
        .map(|s| s.trim().to_string())
        .collect()
}
