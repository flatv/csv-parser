use crate::processor::model::ProcessingStrategy;
use crate::writer::model;
use model::StringWriter;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};

pub struct FileWriter {
    file_name: String,
}

impl FileWriter {
    pub fn new(config: HashMap<String, String>, strategy: &ProcessingStrategy) -> Self {
        let extension = strategy.as_str().to_string();
        let default_filename = format!("output.{}", &extension);
        let file_name = config.get("out").unwrap_or(&default_filename).to_string();
        FileWriter { file_name }
    }
}

impl StringWriter for FileWriter {
    fn write(&self, value: String) -> Result<String, Error> {
        let file_name = &self.file_name;

        if let Some(parent_dir) = std::path::Path::new(file_name).parent() {
            fs::create_dir_all(parent_dir)?;
        }
        let mut file = File::create(file_name)?;
        file.write_all(value.as_bytes())?;
        Ok(file_name.to_string())
    }
}

pub struct StdWriter;

impl StdWriter {
    pub fn new() -> Self {
        StdWriter
    }
}

impl StringWriter for StdWriter {
    fn write(&self, value: String) -> Result<String, Error> {
        Ok(format!("{:?}", &value))
    }
}