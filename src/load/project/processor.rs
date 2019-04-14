//! Processes the stack of Commands and matches them to the appropriate tmux
/// call.
use load::command_v2::Commands;
use load::tmux;
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
///   Commands::Session(Session{...}),
///   Commands::Attach(Attach{...})
/// );
///
/// main(&commands);
/// ```
///
/// commands: The stack of commands to process.
pub fn main(commands: &[Commands]) {
    for c in commands {
        match *c {
            Commands::Session(ref c) => tmux::new_session(&c.name, &c.window_name),
            Commands::Window(ref c) => tmux::new_window(&c.session_name, &c.name),
            Commands::Split(ref c) => tmux::split_window(&c.target),
            Commands::Layout(ref c) => tmux::layout(&c.target, &c.layout),
            Commands::SendKeys(ref c) => tmux::send_keys(&c.target, &c.exec),
            Commands::Attach(ref c) => tmux::attach(&c.name),
            Commands::SelectWindow(ref c) => tmux::select_window(&c.target),
            Commands::SelectPane(ref c) => tmux::select_pane(&c.target),
            Commands::Pre(ref c) => system_calls(&c.exec),
        }
    }
}

fn system_calls(cmd_string: &str) {
    let cmd_array: Vec<&str> = cmd_string.split(' ').collect();
    let (program, args) = cmd_array
        .split_first()
        .expect("Couldn't find args for pre option");

    process::Command::new(program)
        .args(args)
        .output()
        .expect("Didn't execute the process for the pre option.");
}
