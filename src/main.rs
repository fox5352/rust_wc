use ccwc::file_processing::FileProcessingConfig;

fn main() {
    let config = FileProcessingConfig::new(std::env::args()).unwrap();

    // println!("{}", config);
    let formatted_string = match config.run() {
        Ok(data) => data,
        Err(e) => panic!("{}", e)
    };

    println!("{}", formatted_string);
}
