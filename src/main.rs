use std::io::Write;
use std::io;

fn main() {
	loop {
		print!("$ ");
		std::io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("failed to read line");
		print!("{input}")
		// std::process::exit()
	}
}
