//! The interface for interacting with TMUX sessions. All the commands that are
/// built up during the parsing phase get matched to functions here. The
/// functions in this module all build up strings the get passed to a private
/// call. The private call converts them to CStrings and makes an "unsafe" system
/// call. All functions go through this `call` function as a common gateway to
/// system calls and can all be easily logged there.

use libc::system;
use std::ffi::CString;
use std::process::{Command, ExitStatus};

/// The program to call commands on.
static TMUX_NAME: &'static str = "tmux";

/// The gateway to calling any functions on tmux. All public functions in this
/// module will be fed through this `call` function. This functions purpose is to
/// turn string based tmux function calls in to a `CString` and call out to the
/// system with it.
///
/// command: The command we will send to tmux on the host system for execution.
///
/// # Examples
///
/// ```
/// call("new-window -t muxed -c ~/Projects/muxed/".to_string());
/// ```
///
/// # Safety
///
/// This function will make an `unsafe` call out to the system to execute tmux
/// related commands.
fn call(command: String) -> () {
    let line = format!("{} {}", TMUX_NAME, command);
    let system_call = CString::new(line.clone()).unwrap();
    //println!("{}", line.clone());
    unsafe { system(system_call.as_ptr()); };
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
/// session_name: The active tmux session name.
pub fn attach(session_name: &String) -> () {
    call(format!("attach -t {}", session_name));
}

/// New session is the first call made in any sequence of commands. It initiates
/// a new tmux session with a default first window name.
/// *Why a randomized first window name?*
///   This is because the `new-session` tmux command does not support the `-c`
/// flag as `new-window` and `split-window` do. So if the first window requires
/// being changed to a default directory `send-keys` must be used after the
/// session and first window are opened. This means the first window is treated in
/// a different code path than any other window that is opened.
/// Instead I've opted to open the session with a randomized (no conflicts)
/// named window, and we close this window out before attaching the user. This
/// allows for us to treat all user defined windows the same.
///
/// session_name: The name of the session being created.
/// tmp_name: The named first window that we will close out.
///
/// # Examples
///
/// ```
/// use rand::random;
///
/// let session_name = "muxed".to_string();
/// let tmp_window_name = random::<u16>().to_string();
/// tmux::new_session(session_name, tmp_window_name);
/// ```
///
/// session_name: The active tmux session name.
/// tmp_name: The name for the temp initial window that is created with a new
/// session.
pub fn new_session(session_name: &String, tmp_name: &String) -> () {
    call(format!("new -d -s {} -n {}", session_name, tmp_name));
}

/// Split window acts as the command to target named windows in a session to
/// send a make separate a single pane into two panes. Split window should target
/// a specific pane even if only a single pane exists.
///
/// # Examples
///
/// ```
/// let target = "muxed:cargo.0".to_string();
/// tmux::split_window(target, None);
/// ```
///
/// target: A string represented by the {named_session}:{named_window}.{pane}
/// root: An `Option<String>` passed to the -c argument to change the current
/// directory.
pub fn split_window(target: &String, root: &Option<String>) -> () {
    call(match root {
        &Some(ref r) => format!("split-window -t {} -c {}", target, r),
        &None        => format!("split-window -t {}", target)
    });
}

/// New window opens a new window in the named session with the provided window
/// name.
///
/// # Examples
///
/// ```
/// tmux::new_window("muxed".to_string(), "vim".to_string(), None);
/// ```
///
/// session_name: The active tmux session name.
/// window_name: The desired window name for the new window.
/// root: An `Option<String>` passed to the -c argument to change the current
/// directory.
pub fn new_window(session_name: &String, window_name: &String, root: &Option<String>) -> () {
    call(match root {
        &Some(ref r) => format!("new-window -t {} -n {} -c {}", session_name, window_name, r),
        &None        => format!("new-window -t {} -n {}", session_name, window_name)
    });
}

/// The layout function will adjust the layout of a tmux window that already has
/// multiple panes. Currently it will only accept standard tmux layout types.
///
/// Possible layout options: even-horizontal, even-vertical, main-horizontal,
/// main-vertical, or tiled.
///
/// # Examples
///
/// ```
/// tmux::layout("muxed:cargo.0".to_string(), "main-vertical".to_string(), None);
/// ```
///
/// target: A string represented by the {named_session}:{named_window}.{pane}
/// layout: The predefined tmux named layout.
pub fn layout(target: &String, layout: &String) -> () {
    call(format!("select-layout -t {} {}", target, layout));
}

/// Send Keys executes literal key commands in specified target windows. This is
/// the execution method to perform actions like tailing logs, starting servers,
/// or opening editors.
///
/// `KPEnter` represent the keyboard press of the "Enter" key. So the execution
/// is a user input command followed by "enter"
///
/// # Examples
///
/// ```
/// tmux::send_keys("muxed:cargo.0".to_string(), "vim .".to_string());
/// ```
///
/// target: A string represented by the {named_session}:{named_window}.{pane}
/// exec: The system command to be executed in a particular pane.
pub fn send_keys(target: &String, exec: &String) -> () {
    call(format!("send-keys -t {} '{}' KPEnter", target, exec));
}

/// Kill Window is only used to elimiate the default window opened with a
/// randomized name when a new session is started.
///
/// # Examples
///
/// ```
/// tmux::kill_window("muxed:cargo".to_string());
/// ```
///
/// target: A string represented by the {named_session}:{named_window}
/// exec: The system command to be executed in a particular pane.
pub fn kill_window(target: &String) -> () {
    call(format!("kill-window -t {}", target));
}

pub fn list_windows(target: &String) -> ExitStatus {
    let output = Command::new("tmux")
                     .arg("list-windows")
                     .arg("-t")
                     .arg(target)
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    output.status
}
