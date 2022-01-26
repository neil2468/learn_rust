

struct Person {
	name: String,
	age: isize,
}

fn main() {


	let bob = Person {
		name: String::from("Bob"),
		age: 42,
	};

	println!("Name = {}", bob.name);
	println!("Age = {}", bob.age);

	println!("struct = {}", bob);


}
