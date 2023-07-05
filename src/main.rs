use std::{env, process};

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config_vars = Config::new(&args).unwrap_or_else(|err| {
        println!("Config::new() Error:\n{}", err);
        process::exit(1)
    });

    if let Err(err) = run(config_vars) {
        println!("Error occurred while parsing file:\n{}", err);
        process::exit(1);
    }
}


