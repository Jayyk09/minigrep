use std::process;
use std::env;
use std::fs;
use std::error::Error;
mod parse;

struct Config {
    query: String,
    file_path: String
}

impl Config {
fn build(
    mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
    args.next();

    let query = args.next().ok_or("Let no query string")?;
    let file_path = args.next().ok_or("Let no query string")?;

    Ok(Config { query, file_path })
}
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Trouble parsing arguments: {err}");
        process::exit(1);
    });

    let file_content = read_file(&config.file_path)
        .expect("File unable to be read");
    println!("{}", file_content);
    println!("{}", &config.query);
}


fn read_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_name)?;
    Ok(file_content)
}

