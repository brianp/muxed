//! The interface for interacting with TMUX sessions. All the commands that are
/// built up during the parsing phase get matched to functions here. The
/// functions in this module all build up strings the get passed to a private
/// call. The private call converts them to `CStrings` and makes an "unsafe" system
/// call. All functions go through this `call` function as a common gateway to
/// system calls and can all be easily logged there.
pub mod error;
pub mod target;

use crate::tmux::error::TmuxError;
use libc::system;
use std::ffi::CString;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus, Output};

type Result<T> = std::result::Result<T, TmuxError>;

/// The program to call commands on.
static TMUX_NAME: &str = "tmux";

/// The gateway to calling any functions on tmux. Most public functions in this
/// module will be fed through this `call` function. This safely creates a new
/// thread to execute the command on. We say "Most" public functions will use
/// this as `attach` specificaly does not use it.
///
/// args: The command we will send to tmux on the host system for execution.
///
/// # Examples
///
/// ```rust
/// extern crate load;
/// use load::tmux::call;
///
/// let _ = call(&["new-window", "-t", "muxed-test", "-c", "~/Projects/muxed/"]);
/// let _ = call(&["kill-session", "-t", "muxed-test"]);
/// ```
pub fn call(args: &[&str]) -> Result<Output> {
    //println!("{:?}", &args);
    Command::new(TMUX_NAME)
        .args(args)
        .output()
        .map_err(TmuxError::Io)
}

/// Has session is used firgure out if a named session is already running.
///
/// `target`: A string represented by the `{named_session}`
///
/// # Examples
///
/// ```rust
/// extern crate load;
/// use load::tmux;
///
/// let session = tmux::has_session("muxed");
///
/// assert!(!session);
/// ```
pub fn has_session(target: &str) -> bool {
    match call(&["has-session", "-t", target]) {
        Ok(output) => output.status.success(),
        _ => false,
    }
}

/// Read the tmux config and return a config object
///
/// # Examples
///
/// ```rust
/// extern crate load;
/// use load::tmux;
///
/// tmux::get_config();
/// ```
pub fn get_config() -> Result<String> {
    let output = call(&[
        "start-server",
        ";",
        "show-options",
        "-g",
        ";",
        "show-options",
        "-g",
        "-w",
    ])
    .map_err(|_| TmuxError::Config)?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Attach is called as the last function in a set of commands. After the tmux
/// env has been setup by all previous commands this attaches the user to their
/// daemonized tmux session.
///
/// # Examples
///
/// `session_name: The active tmux session name.
///
/// ```rust,no_run
/// extern crate load;
/// use load::tmux;
///
/// tmux::attach(&["muxed"]);
/// ```
pub fn attach(args: &[&str]) -> Result<Output> {
    let arg_string = [&[TMUX_NAME], args].concat().join(" ");
    let system_call = CString::new(arg_string).map_err(TmuxError::Attach)?;
    // println!("{:?}", arg_string.clone());
    unsafe {
        let output = system(system_call.as_ptr());

        Ok(Output {
            status: ExitStatus::from_raw(output),
            stdout: vec![],
            stderr: vec![],
        })
    }
}
