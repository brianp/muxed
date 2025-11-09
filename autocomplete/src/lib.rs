use common::args::Args;
use std::env;

mod bash;
mod error;
mod fish;
mod zsh;

use crate::error::AutocompleteError;

type Result<T> = std::result::Result<T, AutocompleteError>;

pub fn autocomplete(_args: Args) -> Result<()> {
    let shell = detect_shell()?;
    println!("Detected shell: {}", shell);

    match shell.as_str() {
        "bash" => bash::install()?,
        "zsh" => zsh::install()?,
        "fish" => fish::install()?,
        _ => return Err(AutocompleteError::ShellNotSupported),
    }

    Ok(())
}

fn detect_shell() -> Result<String> {
    let shell_path = env::var("SHELL")?;
    let shell_name = shell_path.split('/').next_back().unwrap_or("unknown");
    Ok(shell_name.to_string())
}
