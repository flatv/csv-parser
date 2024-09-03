use clap::{App, Arg, ValueEnum};
use super::config::*;
use crate::processor::model::ProcessingStrategy;

pub fn setup_arg_matches(static_config: &ApplicationConfig) -> clap::ArgMatches {
    App::new(static_config.name.as_str())
        .version(static_config.version.as_str())
        .author(static_config.author.as_str())
        .about(static_config.about.as_str())
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the CSV file to parse")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("strategy")
                .short('s')
                .long("strategy")
                .value_name("STRATEGY")
                .help("Sets the processing strategy")
                .takes_value(true)
                .required(false)
                .possible_values(
                    ProcessingStrategy::value_variants()
                        .into_iter()
                        .map(|s| s.as_str()),
                )
                .case_insensitive(true)
                .default_missing_value("sql")
                .default_value("sql"),
        )
        .arg(
            Arg::new("table")
                .long("table")
                .value_name("TABLE")
                .help("Sets the SQL table name")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("generate_id")
                .long("generate_id")
                .value_name("GENERATE_ID_FILE")
                .help("Path to a YAML file with column-sequence mappings\n\
                       Format: \n\
                       sequences:\n\
                         column_name1: sequence_name1\n\
                         column_name2: sequence_name2")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("out")
                .long("output")
                .value_name("OUTPUT_FILE_PATH")
                .help("Path to a generated output file")
                .takes_value(true)
                .required(false),
        )
        .get_matches()
}
