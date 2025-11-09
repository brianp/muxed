use common::args::Args;
use std::env;

mod bash;
mod error;
mod fish;
mod zsh;

use crate::error::AutocompleteError;

type Result<T> = std::result::Result<T, AutocompleteError>;

/// Detects the user's current shell and installs shell autocompletion support.
///
/// Supports installing autocomplete for "bash", "zsh", and "fish" shells.
/// Returns an error if the shell is not supported, or if installation fails for any reason.
///
/// Caveat: It doesn't detect the running shell right now. It detects the
/// configured user login shell.
///
/// # Arguments
/// * `_args` - Command-line arguments relevant to the autocomplete process.
///
/// # Returns
/// * `Result<()>` - Ok on successful installation, or `AutocompleteError` if an error occurs.
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

/// Detects the user's configured login shell by inspecting the SHELL environment variable.
///
/// This function does NOT detect the currently running shell process. Instead, it retrieves the shell path
/// specified by the SHELL environment variable, which typically points to the user's default login shell as set
/// in their system configuration (e.g., `/bin/zsh`, `/bin/bash`). The shell name is extracted from the end of this path.
///
/// # Returns
/// * `Result<String>` - The name of the configured login shell on success, or an `AutocompleteError` if the SHELL variable
///   is missing or invalid.
///
/// # Example
/// If SHELL is `/bin/zsh`, this will return `"zsh"`.fn detect_shell() -> Result<String> {
fn detect_shell() -> Result<String> {
    let shell_path = env::var("SHELL")?;
    let shell_name = shell_path.split('/').next_back().unwrap_or("unknown");
    Ok(shell_name.to_string())
}
