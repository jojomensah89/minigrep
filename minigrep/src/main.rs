use minigrep;
use std::env;
use std::process;

//  cargo run -- to poem.txt > output.txt
fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = minigrep::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // println!("Searching for {}", config.query);
    // println!("in file  {}", config.file_path);
    // println!("{:?}", args);

    // Read file
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
