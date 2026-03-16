use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{Command, Stdio};
mod builtins;
use builtins::*;

struct Input {
    command: String,
    args: Vec<String>,
}

type BuiltinFn = fn(&[String]) -> Result<(), String>;

fn main() {
    loop {
        let builtins = get_builtins();
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

        let (cmd, args) = alias(input.command, input.args);

        if let Some(&builtin) = builtins.get(cmd.as_str()) {
            match builtin(&args) {
                Ok(()) => {}
                Err(err) => eprintln!("{}", err),
            }
        } else {
            run_command(cmd, args);
        }
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

fn run_command(cmd: String, args: Vec<String>) {
    match Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(status) => {
            if !status.success() {
                match status.code() {
                    Some(code) => eprintln!("process exited with code {}", code),
                    None => eprintln!("process terminated by signal"),
                }
            }
        }
        Err(err) => {
            eprintln!("error running command: {}", err);
        }
    }
}

fn get_builtins() -> HashMap<&'static str, BuiltinFn> {
    let mut builtins: HashMap<&str, BuiltinFn> = HashMap::new();
    builtins.insert("cd", cd);
    builtins.insert("exit", exit);
    builtins
}

// common bash aliases for ease of use
fn alias(cmd: String, args: Vec<String>) -> (String, Vec<String>) {
    match cmd.as_str() {
        "ls" => {
            let mut alias_args = vec![
                "-F".to_string(),
                "--color=auto".to_string(),
                "--show-control-chars".to_string(),
            ];
            alias_args.extend(args);
            (cmd, alias_args)
        }
        "grep" | "egrep" | "fgrep" => {
            let mut alias_args = vec!["--color=auto".to_string()];
            alias_args.extend(args);
            (cmd, alias_args)
        }
        "rm" | "cp" | "mv" => {
            let mut alias_args = vec!["-i".to_string()];
            alias_args.extend(args);
            (cmd, alias_args)
        }
        "df" | "du" => {
            let mut alias_args = vec!["-h".to_string()];
            alias_args.extend(args);
            (cmd, alias_args)
        }
        _ => (cmd, args),
    }
}
