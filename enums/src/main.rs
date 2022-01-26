fn main() {

	println!("{}", foo(100));

}


fn foo(x: usize) -> usize {
	match x {
		100 => 10,
		90 => 9,
		_ => 0,
	}
}




