//! The interface for interacting with TMUX sessions. All the commands that are
/// built up during the parsing phase get matched to functions here. The
/// functions in this module all build up strings the get passed to a private
/// call. The private call converts them to `CStrings` and makes an "unsafe" system
/// call. All functions go through this `call` function as a common gateway to
/// system calls and can all be easily logged there.
pub mod config;
pub mod target;

use libc::system;
use std::ffi::CString;
use std::io;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus, Output};

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
/// ```
/// let _ = call(&["new-window", "-t", "muxed", "-c", "~/Projects/muxed/"]);
/// ```
pub fn call(args: &[&str]) -> Result<Output, io::Error> {
    //println!("{:?}", &args);
    Command::new(TMUX_NAME).args(args).output()
}

/// Has session is used firgure out if a named session is already running.
///
/// # Examples
///
/// ```
/// tmux::has_session("muxed".to_string());
/// => ExitStatus
/// ```
///
/// `target`: A string represented by the `{named_session}`
pub fn has_session(target: &str) -> ExitStatus {
    let output =
        call(&["has-session", "-t", target]).expect("failed to see if the session existed");
    output.status
}

/// Read the tmux config and return a config object
///
/// # Examples
///
/// ```
/// tmux::get_config();
/// => "some-option false\npane-base-index 0"
/// ```
pub fn get_config() -> String {
    let output = call(&["start-server", ";", "show-options", "-g", ";", "show-options", "-g", "-w"])
      .expect("couldn't get tmux options");
    String::from_utf8_lossy(&output.stdout).to_string()
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
pub fn attach(args: &[&str]) -> Result<Output, io::Error> {
    let arg_string = [&[TMUX_NAME], &args[..]].concat().join(" ");
    let system_call = CString::new(arg_string).unwrap();
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
