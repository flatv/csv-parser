use config::config::CliConfig;
use csv_handler::{config, processor};
use processor::file_processor;

fn main() {
    let cli_config = CliConfig::new();
    
    match file_processor::process_file(cli_config) {
        Ok(query) => println!("File processed successfully\n{}", query),
        Err(e) => eprintln!("Error processing file: {:?}", e),
    }
}

