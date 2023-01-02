use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Collect args
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // if minigrep::run returns an error....
    if let Err(e) = minigrep::run(config) {
        // --snip--
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
