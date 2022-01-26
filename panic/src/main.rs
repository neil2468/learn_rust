use std::fs::File;

fn main() {
   	foo().unwrap();
}


fn foo() -> std::io::Result<()> {
	println!("1");
    let f = File::open("hello.txt")?;
	println!("2");

	Ok(())
}