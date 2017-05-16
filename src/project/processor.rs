//! Processes the stack of Commands and matches them to the appropriate tmux
/// call.

use command::Command;
use tmux;
use std::process;

/// Processing of the commands. A simple match occurs to handle the tmux calls
/// needed based off the command provided. The commands are processed first in,
/// first out. So simply looping the Vec is enough to execute all needed
/// instructions.
///
/// # Example
///
/// ```
/// let commands: Vec<Command> = vec!(
///   Command::Session(Session{...}),
///   Command::Attach(Attach{...})
/// );
///
/// main(&commands);
/// ```
///
/// commands: The stack of commands to process.
pub fn main(commands: &Vec<Command>) -> () {
    for c in commands {
        match *c {
            Command::Session(ref c)      => tmux::new_session(&c.name, &c.window_name),
            Command::Window(ref c)       => tmux::new_window(&c.session_name, &c.name),
            Command::Split(ref c)        => tmux::split_window(&c.target),
            Command::Layout(ref c)       => tmux::layout(&c.target, &c.layout),
            Command::SendKeys(ref c)     => tmux::send_keys(&c.target, &c.exec),
            Command::Attach(ref c)       => tmux::attach(&c.name),
            Command::SelectWindow(ref c) => tmux::select_window(&c.target),
            Command::SelectPane(ref c)   => tmux::select_pane(&c.target),
            Command::Pre(ref c)          => system_calls(&c.exec)
        }
    };
}

fn system_calls(cmd_string: &str) -> () {
    let cmd_array: Vec<&str> = cmd_string.split(' ').collect();
    let (program, args) = cmd_array.split_first().expect("Couldn't find args for pre option");

    process::Command::new(program)
        .args(args)
        .output()
        .expect("Didn't execute the process for the pre option.");
}
