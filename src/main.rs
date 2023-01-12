use std::{env, process};

use tinygrep_rs::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let cfg = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = run(cfg) {
        eprintln!("{e}");
        process::exit(1);
    }
}