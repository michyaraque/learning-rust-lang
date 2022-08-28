use std::env;

use minigrep::Config;

//cargo run archivo.txt search_string
fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let config = Config::new(&args);
    println!("File: {}\nSearch: {}", config.filename, config.query);

    minigrep::run(config);
}

