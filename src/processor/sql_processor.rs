use crate::config::config::ProcessConfig;
use crate::format::handler_types::ChainHandler;
use crate::processor::model::Processor;
use crate::reader::csv_reader::CsvRow;
use std::collections::HashMap;

pub struct SqlProcessor;

impl Processor for SqlProcessor {
    fn process(&self, config: Box<dyn ProcessConfig>, rows: &Vec<CsvRow>) -> String {
        let params = config.get_main_params();
        let column_seq = config.get_additional_params();
        let table = params.get("table").unwrap();

        generate_insert_query(table, rows, column_seq)
    }
}

fn generate_insert_query(table_name: &str, rows: &Vec<CsvRow>, sequence_map: HashMap<String, String>) -> String {
    let headers = &rows.get(0).unwrap().headers.names;
    let columns: Vec<&str> = headers.iter().map(|h| h.as_str()).collect();
    let chain_handler = ChainHandler::new();

    let id_columns: Vec<&str> = headers.iter()
        .filter(|&header| header.contains("id"))
        .map(|h| h.as_str())
        .collect();

    let mut values = Vec::new();

    for row in rows {
        let value: Vec<String> = headers
            .iter()
            .map(|header| {
                if !sequence_map.is_empty() && id_columns.contains(&header.as_str()) {
                    generate_id_value(rows, header, &sequence_map, &chain_handler)
                } else {
                    chain_handler.handle(row.columns.get(header).unwrap_or(&"NULL".to_string()))
                }
            })
            .collect();
        values.push(format!("({})", value.join(", ")));
    }

    format!(
        "INSERT INTO {} ({}) VALUES {};",
        table_name,
        columns.join(", "),
        values.join(", ")
    )
}

fn generate_id_value(
    rows: &Vec<CsvRow>,
    header: &str,
    sequence_map: &HashMap<String, String>,
    chain_handler: &ChainHandler,
) -> String {
    if let Some(sequence) = sequence_map.get(header) {
        let all_empty = rows.iter()
            .all(|r| r.columns.get(header).unwrap_or(&"".to_string()).is_empty());
        if all_empty {
            format!("nextval('{}')", sequence)
        } else {
            chain_handler.handle(rows.get(0).unwrap().columns.get(header).unwrap_or(&"NULL".to_string()))
        }
    } else {
        chain_handler.handle(rows.get(0).unwrap().columns.get(header).unwrap_or(&"NULL".to_string()))
    }
}
