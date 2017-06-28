fn main() {
	for n in 0..10 {
		print!("{} ", fibonacci(n));
	}

	println!("");
}

fn fibonacci(n: u64) -> u64 {

	let result: u64;

	if n <= 1 {
		result = n;
	} else {
		result = fibonacci(n - 1) + fibonacci( n - 2);
	}

	result
}
