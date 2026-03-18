use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{Command, Stdio};
mod builtins;
mod util;
use crate::util::colours::*;
use builtins::*;
use rustyline::DefaultEditor;
use std::cell::RefCell;

struct Input {
    command: String,
    args: Vec<String>,
    background: bool,
}

type BuiltinFn = fn(&[String]) -> Result<(), String>;

thread_local! {
    pub static HISTORY: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

const PROMPT_CHAR: &str = "> ";

fn main() {
    let builtins = get_builtins();
    let mut r1 = DefaultEditor::new().unwrap();
    loop {
        let path = std::env::current_dir()
            .unwrap()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let prompt = format!(".../{} {}", path, PROMPT_CHAR);

        match r1.readline(&prompt) {
            Ok(line) => {
                r1.add_history_entry(&line).ok();
                if !line.trim().is_empty() {
                    HISTORY.with(|h| h.borrow_mut().push(line.trim().to_string()));
                }
                let input = match parse_input(&line.trim()) {
                    Some(input) => input,
                    None => continue,
                };

                let (cmd, args) = alias(input.command, input.args);
                let background = input.background;

                if let Some(&builtin) = builtins.get(cmd.as_str()) {
                    match builtin(&args) {
                        Ok(()) => {}
                        Err(err) => eprintln!("{}", err),
                    }
                } else {
                    run_command(cmd, args, background);
                }
            }
            Err(rustyline::error::ReadlineError::Eof) => break, // ctrl-d
            Err(rustyline::error::ReadlineError::Interrupted) => continue, // ctrl-c
            Err(e) => eprintln!("error: {}", e),
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

    let mut background = false;

    if let Some(last) = args.last() {
        if last == "&" {
            background = true;
            args.pop();
        }
    }

    Some(Input {
        command,
        args,
        background,
    })
}

fn run_command(cmd: String, args: Vec<String>, background: bool) {
    if !background {
        match Command::new(cmd)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
        {
            Ok(status) => {
                if !status.success() {
                    match status.code() {
                        Some(code) => {
                            eprintln!("{BOLD_RED}process exited with code {}{RESET}", code)
                        }
                        None => eprintln!("{BOLD_RED}process terminated by signal{RESET}"),
                    }
                }
            }
            Err(err) => {
                eprintln!("{BOLD_RED}error running command: {}{RESET}", err);
            }
        }
    } else {
        match Command::new(cmd)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(child) => {
                println!("[{}] running in background", child.id())
            }
            Err(err) => {
                eprintln!("{BOLD_RED}error running command: {}{RESET}", err);
            }
        }
    }
}

fn get_builtins() -> HashMap<&'static str, BuiltinFn> {
    let mut builtins: HashMap<&str, BuiltinFn> = HashMap::new();
    builtins.insert("cd", cd);
    builtins.insert("history", history);
    builtins.insert("!!", bangbang);
    builtins.insert("exit", exit);
    builtins.insert("help", help);
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
