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
    if args.is_empty() {
        return Ok(());
    }

    let path = match args[0].as_str() {
        "~" => dirs::home_dir()
            .ok_or_else(|| format!("{BOLD_RED}cd: could not determine home directory{RESET}"))?,
        ".." => std::env::current_dir()
            .map_err(|e| format!("{BOLD_RED}cd: {}{RESET}", e))?
            .parent()
            .ok_or_else(|| format!("{BOLD_RED}cd: no parent directory{RESET}"))?
            .to_path_buf(),
        "-" => {
            return Err(format!(
                "{YELLOW}cd: previous directory not yet implemented{RESET}"
            ));
        }
        "/" => PathBuf::from("/"),
        _ => PathBuf::from(&args[0]),
    };

    std::env::set_current_dir(path).map_err(|e| format!("{BOLD_RED}cd: {}{RESET}", e))?;
    Ok(())
}
