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
