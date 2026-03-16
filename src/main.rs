use std::io::{self, Write};

fn main() {
	loop {
		print!("$ ");
		std::io::stdout().flush().unwrap();
		let mut input = String::new();
		let bytes = io::stdin().read_line(&mut input).expect("failed to read line");
		
		if bytes == 0 {
			break
		}
        
		print!("{}", input)
	}
}
