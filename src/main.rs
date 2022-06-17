use std::{env, process};

use calculator::Operation;

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation = Operation::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = calculator::run(&operation) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
