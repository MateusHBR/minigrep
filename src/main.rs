use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let (Some(query), Some(file_path)) = (args.get(1), args.get(2)) else {
        eprintln!("Expected args <query> <file-path>");
        std::process::exit(1);
    };

    let Ok(content) = fs::read_to_string(file_path) else {
        eprintln!("Could not read file \"{}\"", file_path);
        std::process::exit(1);
    };

    println!("Searching for \"{}\" on \"{}\" ...", query, file_path);

    println!("Content: \n{}", content);
}
