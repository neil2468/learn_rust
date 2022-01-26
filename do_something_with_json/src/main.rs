extern crate do_something_with_json;

use std::io::Write;

fn main() {
	let mut stderr = std::io::stderr();

    if let Err(e) = do_something_with_json::run() {
		writeln!(&mut stderr, "Application Error: {}", e).unwrap();
    	std::process::exit(1);
	}
}

