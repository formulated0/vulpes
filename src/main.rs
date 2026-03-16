use std::io::{self, Write};

struct Input {
    command: String,
    args: Vec<String>,
}

fn main() {
    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        // break on EOF (ctrl-d on unix ctrl-z on win)
        if bytes == 0 {
            break;
        }

        let input = match parse_input(&line.trim()) {
            Some(input) => input,
            None => continue,
        };

        println!("{} {:?}", input.command, input.args);
    }
}

fn parse_input(line: &str) -> Option<Input> {
    #[derive(PartialEq, Eq)]
    enum QuoteState {
        None,
        Single,
        Double,
    }

    let mut state = QuoteState::None;
    let mut current_arg = String::new();
    let mut args = vec![];

    for c in line.chars() {
        match c {
            '\'' => {
                if state == QuoteState::None {
                    state = QuoteState::Single;
                } else if state == QuoteState::Single {
                    state = QuoteState::None;
                } else if state == QuoteState::Double {
                    current_arg.push(c)
                }
            }
            '"' => {
                if state == QuoteState::None {
                    state = QuoteState::Double;
                } else if state == QuoteState::Double {
                    state = QuoteState::None;
                } else if state == QuoteState::Single {
                    current_arg.push(c)
                }
            }
            _ if c.is_whitespace() => {
                if state == QuoteState::None {
                    if !current_arg.is_empty() {
                        args.push(current_arg);
                    }
                    current_arg = String::new(); // clears buf
                } else {
                    current_arg.push(c);
                }
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    let command;
    if args.len() > 0 {
        command = args.remove(0);
    } else {
        command = String::new();
    }

    Some(Input { command, args })
}
