//! Processes the stack of Commands and matches them to the appropriate tmux
use libc::system;
/// call.
use load::command::{Command, Commands};
use std::ffi::CString;
use std::process;

/// The program to call commands on.
static TMUX_NAME: &'static str = "tmux";

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
            Commands::Attach(ref c) => attach(&c.name),
            Commands::Layout(ref c) => call(&c.call()),
            Commands::SelectPane(ref c) => call(&c.call()),
            Commands::SelectWindow(ref c) => call(&c.call()),
            Commands::SendKeys(ref c) => call(&c.call()),
            Commands::Session(ref c) => call(&c.call()),
            Commands::Split(ref c) => call(&c.call()),
            Commands::Window(ref c) => call(&c.call()),
        }
    }
}

/// The gateway to calling any functions on tmux. Most public functions in this
/// module will be fed through this `call` function. This safely creates a new
/// thread to execute the command on. We say "Most" public functions will use
/// this as `attach` specificaly does not use it.
///
/// args: The command we will send to tmux on the host system for execution.
///
/// # Examples
///
/// ```
/// let _ = call(&["new-window", "-t", "muxed", "-c", "~/Projects/muxed/"]);
/// ```
fn call(args: &[&str]) -> () { //Result<Output, io::Error> {
    //println!("{:?}", &args);
    process::Command::new(TMUX_NAME).args(args).output();
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

/// Attach is called as the last function in a set of commands. After the tmux
/// env has been setup by all previous commands this attaches the user to their
/// daemonized tmux session.
///
/// # Examples
///
/// ```
/// let session_name = "muxed".to_string();
/// tmux::attach(muxed);
/// ```
/// `session_name: The active tmux session name.
pub fn attach(session_name: &str) {
    let line = format!(
        "{} attach -t '{}' {}",
        TMUX_NAME, session_name, ">/dev/null"
    );
    let system_call = CString::new(line.clone()).unwrap();
    //println!("{}", line.clone());
    unsafe {
        system(system_call.as_ptr());
    };
}
