use super::command_line_args;
use crate::processor::model::ProcessingStrategy;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{collections::HashMap, fs::File, io::BufReader};
use log::{log, Level};

pub trait ProcessConfig {
    fn get_main_params(&self) -> HashMap<String, String>;
    fn get_additional_params(&self) -> HashMap<String, String>;
    fn get_write_params(&self) -> HashMap<String, String>;
}
#[derive(Debug, Clone)]
pub struct SqlConfig {
    pub config: HashMap<String, String>,
    pub column_seq: HashMap<String, String>,
    pub write_params: HashMap<String, String>,
}

impl ProcessConfig for SqlConfig {
    fn get_main_params(&self) -> HashMap<String, String> {
        self.config.clone()
    }

    fn get_additional_params(&self) -> HashMap<String, String> {
        self.column_seq.clone()
    }

    fn get_write_params(&self) -> HashMap<String, String> {
        self.write_params.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CliConfig {
    pub sql_config: SqlConfig,
    pub static_config: Config,
    pub strategy: ProcessingStrategy,
    pub file_path: String,
}

impl CliConfig {
    pub fn new() -> Self {
        let static_config = Config::new();

        let matches = command_line_args::setup_arg_matches(&static_config.application);

        let file_path = matches.value_of("file").expect("File path is required");
        let strategy = matches.value_of("strategy").unwrap_or("sql");
        let table = matches.value_of("table").unwrap_or_else(|| {
            match file_path.rfind('.') {
                Some(pos) => &file_path[..pos],
                None => file_path
            }
        });
        let generate_id = matches.value_of("generate_id").unwrap_or_default();
        let out_file = matches.value_of("out").unwrap_or_default();

        let processing_strategy = ProcessingStrategy::from_str(strategy).unwrap_or(ProcessingStrategy::Sql);

        let mut command_line_args_map = HashMap::new();
        command_line_args_map.insert("table".to_string(), table.to_string());

        let column_seq = if !generate_id.is_empty() {
            let file = File::open(generate_id).expect("Failed to open generate_id file");
            let reader = BufReader::new(file);
            let sequence_config: HashMap<String, String> =
                serde_yaml::from_reader(reader).expect("Failed to parse generate_id file");
            sequence_config
        } else {
            HashMap::new()
        };

        let mut write_params = HashMap::new();
        write_params.insert("out".to_string(), out_file.to_string());

        Self {
            sql_config: SqlConfig {
                config: command_line_args_map,
                column_seq,
                write_params,
            },
            static_config,
            strategy: processing_strategy,
            file_path: file_path.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub application: ApplicationConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub name: String,
    pub separator: char,
    pub empty_value: String,
    pub version: String,
    pub author: String,
    pub about: String,
}

impl Config {
    pub fn new() -> Self {
        let file = File::open("static.yaml").expect("Failed to open config file");
        let reader = BufReader::new(file);
        let config: Config =
            serde_yaml::from_reader(reader).expect("Failed to parse config file");

        config
    }
}
