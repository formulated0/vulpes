use std::io::{self, Write};

struct Input<'a> {
    command: &'a str,
    args: Vec<&'a str>,
}

fn main() {
	loop {
		print!("$ ");
		std::io::stdout().flush().unwrap();
		
		let mut line = String::new();
		let bytes = io::stdin().read_line(&mut line).expect("failed to read line");
		
		if bytes == 0 {
			break
		}
		
		if let Some(input) = parse_input(&line) {
            println!("{} {:?}", input.command, input.args);
        }
	}
}

fn parse_input(line: &str) -> Option<Input<'_>> {
    let mut parts = line.split_whitespace();
    let command = parts.next()?;

    Some(Input {
        command,
        args: parts.collect(),
    })
}