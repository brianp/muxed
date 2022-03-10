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
/// this as `attach` specifically does not use it.
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
pub fn call(args: &[&str]) -> Result<Output, io::Error> {
    //println!("{:?}", &args);
    Command::new(TMUX_NAME).args(args).output()
}

/// Has session is used figure out if a named session is already running.
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
/// assert!(!session.success());
/// ```
pub fn has_session(target: &str) -> ExitStatus {
    let output =
        call(&["has-session", "-t", target]).expect("failed to see if the session existed");
    output.status
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
pub fn get_config() -> String {
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
    .expect("couldn't get tmux options");
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub struct Session {
    pub name: String,
    pub id: String,
    pub client_attached: usize,
    pub created_at: u64,
    pub last_attached: u64,
}

pub struct SessionList {
    pub sessions: Vec<Session>,
}

impl SessionList {
    pub fn new(sessions: Vec<Session>) -> SessionList {
        SessionList { sessions }
    }

    pub fn has_session(&self, name: &str) -> bool {
        self.sessions.iter().any(|s| s.name == name)
    }

    pub fn is_attached(&self, name: &str) -> bool {
        self.sessions
            .iter()
            .any(|s| s.name == name && s.client_attached > 0)
    }
}

impl Session {
    pub fn from_formatted_str(formatted: &str) -> Session {
        let mut split = formatted.split(' ');
        let name = split.next().unwrap().to_string();
        let id = split.next().unwrap().to_string();
        let client_attached = split
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Bad client_attached");
        let created_at = split
            .next()
            .unwrap()
            .parse::<u64>()
            .expect("Bad created_at");
        let last_attached = split
            .next()
            .unwrap()
            .parse::<u64>()
            .expect("Bad last_attached");

        Session {
            name,
            id,
            client_attached,
            created_at,
            last_attached,
        }
    }
}

/// Read the current tmux sessoins from the server
///
/// # Examples
///
/// ```rust
/// extern crate load;
/// use load::tmux;
///
/// let _: tmux::SessionList = tmux::get_sessions();
/// ```
pub fn get_sessions() -> SessionList {
    let output = call(&[
        "list-sessions",
        "-F",
        "#{session_name} #{session_id} #{session_attached} #{session_created} #{session_last_attached}",
    ])
    .expect("couldn't get tmux sessions");
    let sessions = String::from_utf8_lossy(&output.stdout).to_string();
    let sessions = sessions
        .trim()
        .split('\n')
        .map(Session::from_formatted_str)
        .collect::<Vec<_>>();

    SessionList { sessions }
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
