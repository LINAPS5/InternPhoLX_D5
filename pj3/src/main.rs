use std::io;

fn main() {
	let mut name = String::new();
	println!("Please enter your name: ");
	io::stdin()
		.read_line(&mut name)
		.expect("Data reading failed");

	println!("Hello, {}", name);
}
