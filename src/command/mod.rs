//! The structures used to manage commands sent over to tmux.

/// The Session command is used to fire up a new daemonized session in tmux.
/// name: The Name of a named tmux session.
/// tmp_window_name: The randomized tmp window name created for the first window
/// of the session. This window is closed before attaching.
#[derive(Debug)]
#[derive(Clone)]
pub struct Session {
    pub name: String,
    pub window_name: String
}

/// The Window command is used to identify every new window opened in the tmux
/// session.
/// session_name: The name of the session.
/// name: The named window to be opened.
/// root: An `Option<String>` containing a possible root directory passed to the
/// `-c` arguement.
#[derive(Debug)]
#[derive(Clone)]
pub struct Window {
    pub session_name: String,
    pub name: String
}

/// The Split is used to call split-window on a particular window in the
/// session.
/// target: The target window. In the format `{session}:{window}.{paneIndex}`.
/// root: An `Option<String>` containing a possible root directory passed to the
/// `-c` arguement.
#[derive(Debug)]
#[derive(Clone)]
pub struct Split {
    pub target: String,
    pub root: Option<String>
}

/// The Layout command calls select-layout with a specific pre-defined tmux
/// layout option see `tmux select-layout --help` for more options.
/// target: The target window. In the format `{session}:{window}.{paneIndex}`.
/// layout: The type of layout. ex `main-horizontal`.
#[derive(Debug)]
#[derive(Clone)]
pub struct Layout {
    pub target: String,
    pub layout: String,
}

/// A generic SendKeys command used to send "typed" commands to tmux. This is
/// used to initialize processes or tasks in specific window. Such as starting log
/// tails or running servers.
/// target: The target window. In the format `{session}:{window}.{paneIndex}`.
/// exec: The cli command to be run. ex. `tail -f logs/development.log`.
#[derive(Debug)]
#[derive(Clone)]
pub struct SendKeys {
    pub target: String,
    pub exec: String
}

/// Used to attach to the daemonized session.
/// name: The named session to attach too.
#[derive(Debug)]
#[derive(Clone)]
pub struct Attach {
    pub name: String
}

/// Sends the kill-window command. Used to kill off the randomized first pane
/// opened with the new session.
/// name: the name of the window to kill.
#[derive(Debug)]
#[derive(Clone)]
pub struct KillWindow {
    pub name: String
}

/// The Command enum. Commands represent the series of commands sent to the
/// running tmux process to build a users env. This is an enum to support
/// containing all the commands that require running in a single Vec. This
/// allows a simple process of first in, first out command execution.
#[derive(Debug)]
#[derive(Clone)]
pub enum Command {
    Session(Session),
    Window(Window),
    Split(Split),
    Layout(Layout),
    SendKeys(SendKeys),
    Attach(Attach),
    KillWindow(KillWindow)
}
