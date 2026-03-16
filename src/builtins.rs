use std::path::PathBuf;

pub fn exit(args: &[String]) -> Result<(), String> {
    if args.len() > 1 {
        return Err("exit: too many arguments".to_string());
    }

    let code = if args.is_empty() {
        0
    } else {
        args[0]
            .parse::<i32>()
            .map_err(|_| "exit: argument must be a number")?
    };

    std::process::exit(code);
}

pub fn cd(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Ok(());
    }

    let path = match args[0].as_str() {
        "~" => {
            dirs::home_dir().ok_or_else(|| "cd: could not determine home directory".to_string())?
        }
        ".." => std::env::current_dir()
            .map_err(|e| format!("cd: {}", e))?
            .parent()
            .ok_or_else(|| "cd: no parent directory".to_string())?
            .to_path_buf(),
        "-" => {
            return Err("cd: previous directory not yet implemented".to_string());
        }
        "/" => PathBuf::from("/"),
        _ => PathBuf::from(&args[0]),
    };

    std::env::set_current_dir(path).map_err(|e| format!("cd: {}", e))?;
    Ok(())
}
