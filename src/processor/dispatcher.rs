use crate::config::config::{CliConfig, ProcessConfig};
use crate::processor::model::{ProcessingStrategy, Processor};
use crate::processor::sql_processor::SqlProcessor;

pub struct ProcessDispatcher;

pub struct ProcessContext {
    pub processor: Box<dyn Processor>,
    pub processor_config: Box<dyn ProcessConfig>,
}
impl ProcessDispatcher {
    pub fn dispatch(processing_strategy: &ProcessingStrategy, config: &CliConfig) -> ProcessContext {
        match processing_strategy {
            ProcessingStrategy::Sql => ProcessContext {
                processor: Box::new(SqlProcessor),
                processor_config: Box::new(config.sql_config.clone()),
            },
        }
    }
}

