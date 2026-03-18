use crate::HISTORY;
use crate::util::colours::*;
use std::path::PathBuf;

pub fn exit(args: &[String]) -> Result<(), String> {
    if args.len() > 1 {
        return Err(format!("{YELLOW}exit: too many arguments{RESET}").to_string());
    }

    let code = if args.is_empty() {
        0
    } else {
        args[0]
            .parse::<i32>()
            .map_err(|_| format!("{BOLD_RED}exit: argument must be a number{RESET}"))?
    };

    std::process::exit(code);
}

pub fn cd(args: &[String]) -> Result<(), String> {
    let current = std::env::current_dir().unwrap();

    if args.is_empty() {
        return Ok(());
    }

    let path = match args[0].as_str() {
        "~" => dirs::home_dir()
            .ok_or_else(|| format!("{BOLD_RED}cd: could not determine home directory{RESET}"))?,
        ".." => current
            .parent()
            .ok_or_else(|| format!("{BOLD_RED}cd: no parent directory{RESET}"))?
            .to_path_buf(),
        "-" => std::env::var("OLDPWD")
            .map(PathBuf::from)
            .map_err(|_| format!("{BOLD_RED}cd: OLDPWD not set{RESET}"))?,
        "/" => PathBuf::from("/"),
        _ => PathBuf::from(&args[0]),
    };

    // save current directory before changing
    unsafe {
        std::env::set_var("OLDPWD", current.to_string_lossy().to_string());
    }
    std::env::set_current_dir(path).map_err(|e| format!("{BOLD_RED}cd: {}{RESET}", e))?;
    Ok(())
}

pub fn history(args: &[String]) -> Result<(), String> {
    if !args.is_empty() {
        match args[0].as_str() {
            "-c" | "--clear" => {
                HISTORY.with(|history| {
                    history.borrow_mut().clear();
                });
            }
            "-h" | "--help" => {
                println!("{PURPLE}history{RESET}\nusage: ")
            }
            // last N history entries
            _ => match args[0].parse::<usize>() {
                Ok(n) => {
                    HISTORY.with(|h| {
                        let history = h.borrow();
                        let len = history.len();
                        let start = len.saturating_sub(n);

                        for (i, entry) in history.iter().enumerate().skip(start) {
                            println!("{GREEN}{:>4}{RESET}  {}", i + 1, entry);
                        }
                    });
                }
                Err(_) => {
                    eprintln!("{BOLD_RED}history: invalid argument{RESET}");
                }
            },
        }
    } else {
        HISTORY.with(|h| {
            for (i, entry) in h.borrow().iter().enumerate() {
                println!("{GREEN}{:>4}{RESET}  {}", i + 1, entry);
            }
        });
    }
    Ok(())
}

pub fn help(args: &[String]) -> Result<(), String> {
    println!("{PURPLE}┌─ vulpes help menu ─────────────────────────────┐{RESET}");
    println!(" vulpes supports most system commands by default.\n");

    let commands = vec![
        ("cd", "change directory"),
        ("pwd", "print working directory"),
        ("ls", "list directory contents"),
        ("cat", "display file contents"),
        ("echo", "print text"),
        ("mkdir", "create directory"),
        ("rm", "remove files or directories"),
        ("cp", "copy files or directories"),
        ("mv", "move or rename files"),
        ("exit", "exit the shell"),
        ("help", "show this help menu"),
    ];

    for (cmd, desc) in commands {
        println!("{GREEN} {:12}{RESET} {}", cmd, desc);
    }

    println!("{PURPLE}└────────────────────────────────────────────────┘{RESET}");
    Ok(())
}
