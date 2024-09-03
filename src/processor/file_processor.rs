use crate::processor::dispatcher::ProcessDispatcher;
use crate::writer::model::StringWriter;
use crate::writer::writer::FileWriter;
use crate::{config::config::CliConfig, reader::csv_reader::CsvParser};
use crate::reader::error::ParseError;

pub fn process_file(config: CliConfig) -> Result<String, ParseError> {
    let strategy = config.strategy.clone();
    let file_path = &config.file_path;
    let separator = config.static_config.application.separator;

    let csv_rows = CsvParser::parse_file(file_path, separator)?;
    let process_context = ProcessDispatcher::dispatch(&strategy, &config);
    let processor = process_context.processor;
    let processor_config = process_context.processor_config;

    let string_result = processor.process(processor_config, &csv_rows);

    FileWriter::new(config.sql_config.write_params, &strategy).write(string_result).map_err(ParseError::from)
}
