//! The interface for interacting with TMUX sessions. All the commands that are
/// built up during the parsing phase get matched to functions here. The
/// functions in this module all build up strings the get passed to a private
/// call. The private call converts them to `CStrings` and makes an "unsafe" system
/// call. All functions go through this `call` function as a common gateway to
/// system calls and can all be easily logged there.
pub mod config;

use std::io;
use std::process::{Command, ExitStatus, Output};

/// The program to call commands on.
static TMUX_NAME: &'static str = "tmux";

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
fn call(args: &[&str]) -> Result<Output, io::Error> {
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
