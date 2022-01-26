
extern crate greprs;

// Bring into scope
use std::env;
use std::process;
use std::io::prelude::*;

use greprs::Config;

fn main() {
    let mut stderr = std::io::stderr();
    let args: Vec<String> = env::args().collect();

	let config = Config::new(&args).unwrap_or_else(|err| {

		writeln!(
			&mut stderr,
			"Problem parsing arguments: {}",
			err			
		).expect("Failed to write to stderr");

        process::exit(1);
    });

    if let Err(e) = greprs::run(config) {

		writeln!(
			&mut stderr,
			"Application Error: {}",
			e			
		).expect("Failed to write to stderr");

        process::exit(1);
    }
}

