use std::env;
use std::fmt::Display;
use std::process;
use minigrep::Config;

fn main() {
    let v: Vec<String> = env::args().collect();
    let config = Config::new(&v).unwrap_or_else(| err | {
        println!("{}", err);
        process::exit(1)
    });
    if let Err(e) = minigrep::run(config) {
        println!("{}", e);
        process::exit(1)
    }
}