fn main() {


	let x: f64 = 20.0;

	println!("{}°C \t=\t{}°F", x, convert_to_farenheit(x));
	println!("{}°F \t=\t{}°C", x, convert_to_celcius(x));
}


fn convert_to_farenheit(celcius: f64) -> f64 {
	( celcius * 1.8 ) + 32.0 
}

fn convert_to_celcius(farenheit: f64) -> f64 {
	( farenheit - 32.0 ) / 1.8
}