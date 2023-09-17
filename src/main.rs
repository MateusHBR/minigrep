use std::{env, process};

use minigrep::ConfigBuilder;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = ConfigBuilder::new().with_args(&args).build();
    let config = match config {
        Ok(config) => config,
        Err(err) => {
            eprintln!("problems parsing arguments: \"{}\"", &err);
            process::exit(1);
        }
    };
    println!(
        "\nSearching for \"{}\" on \"{}\" ...\n",
        &config.query, &config.file_path
    );

    match minigrep::run(&config) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("File error: \"{}\"", &err);
            process::exit(1);
        }
    };
}
