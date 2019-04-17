//! Processes the stack of Commands and matches them to the appropriate tmux call.

use libc::system;
use load::command::{Command, Commands};
use std::ffi::CString;
use std::io;
use std::process;

/// Processing of the commands. A simple match occurs to handle the tmux calls
/// needed based off the command provided. The commands are processed first in,
/// first out. So simply looping the Vec is enough to execute all needed
/// instructions.
///
/// # Example
///
/// ```
/// let commands: Vec<Commands> = vec!(
///     Commands::Session(Session{...}),
///     Commands::Attach(Attach{...})
/// );
///
/// main(&commands);
/// ```
///
/// commands: The stack of commands to process.
pub fn main(commands: &Vec<Commands>) {
    for c in commands {
        match *c {
            Commands::Pre(ref c) => system_calls(&c.exec),
            Commands::Attach(ref c) => c.call(),
            Commands::Layout(ref c) => c.call(),
            Commands::SelectPane(ref c) => c.call(),
            Commands::SelectWindow(ref c) => c.call(),
            Commands::SendKeys(ref c) => c.call(),
            Commands::Session(ref c) => c.call(),
            Commands::Split(ref c) => c.call(),
            Commands::Window(ref c) => c.call(),
        };
    }
}

fn system_calls(cmd_string: &str) -> Result<process::Output, io::Error> {
    let cmd_array: Vec<&str> = cmd_string.split(' ').collect();
    let (program, args) = cmd_array
        .split_first()
        .expect("Couldn't find args for pre option");

    process::Command::new(program)
        .args(args)
        .output()
}
