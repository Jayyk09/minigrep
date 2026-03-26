use std::{env, io};
use std::fs;

struct Config {
    query: String,
    file_path: String
}

impl Config {
fn build(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args);
    let file_content = read_file(&config.file_path)
        .expect("File unable to be read");
    println!("{}", file_content);
    println!("{}", &config.query);
}


fn read_file(file_name: &str) -> Result<String, io::Error> {
    let file_content = fs::read_to_string(file_name)?;
    Ok(file_content)
}

