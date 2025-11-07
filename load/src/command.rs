//! The structures used to manage commands sent over to tmux.

use crate::common::tmux::Target;
use crate::tmux;
use crate::tmux::error::TmuxError;
use common::DEBUG;
use std::path::PathBuf;
use std::process::Output;
use std::{process, str};

pub trait Command {
    fn call(&self) -> Result<Output, TmuxError> {
        if DEBUG.load() {
            println!("{:?}", &self.args());
        };

        tmux::call(&self.args())
    }

    fn args(&self) -> Vec<&str>;
}

/// The Session command is used to fire up a new daemonized session in tmux.
/// `name`: The Name of a named tmux session.
/// `window_name`: The Name of the first window.
/// `root_path`: The root directory for the tmux session.
#[derive(Debug, Clone)]
pub struct Session {
    pub target: Target,
    pub window_name: String,
    pub root_path: Option<PathBuf>,
}

impl Session {
    pub fn new<S: AsRef<str> + Into<String>>(
        name: S,
        window_name: S,
        root_path: Option<PathBuf>,
    ) -> Session {
        Session {
            target: Target::new(name.into(), None, None),
            window_name: window_name.into(),
            root_path,
        }
    }
}

// TODO: Real logic exists here. Test it!
impl Command for Session {
    fn args(&self) -> Vec<&str> {
        let args: Vec<&str> = vec![
            "new",
            "-d",
            "-s",
            &self.target.combined,
            "-n",
            &self.window_name,
        ];

        match self.root_path.as_ref().and_then(|p| p.to_str()) {
            Some(path) => [&args[..], &["-c", path]].concat(),
            None => args,
        }
    }
}

/// The Window command is used to identify every new window opened in the tmux
/// session.
/// `session_name`: The name of the session followed by a ':' to state we want
/// to open a new window in the next open index.
/// `name`: The named window to be opened.
/// `path`: An `Option<PathBuf>` containing a possible root directory passed to the
/// `-c` arguement.
/// TODO: Turn session_name into a SessionTarget. Remove session_name_arg. Store
/// the mutated value ':' in the SessionTarget. Convert SessionTarget from &str
/// to Rc<String>.
#[derive(Debug, Clone)]
pub struct Window {
    pub name: String,
    pub path: Option<PathBuf>,
    pub target: Target,
}

impl Window {
    pub fn new<S: AsRef<str> + Into<String>>(
        name: S,
        target: Target,
        path: Option<PathBuf>,
    ) -> Window {
        Window {
            name: name.into(),
            path,
            target,
        }
    }
}

impl Command for Window {
    fn args(&self) -> Vec<&str> {
        let args: Vec<&str> = vec!["new-window", "-t", &self.target.combined, "-n", &self.name];

        match self.path.as_ref().and_then(|p| p.to_str()) {
            Some(path) => [&args[..], &["-c", path]].concat(),
            None => args,
        }
    }
}

/// The Split is used to call split-window on a particular window in the
/// session.
/// `target`: The target window. In the format `{session}:{window}.{paneIndex}`.
/// `path`: An `Option<PathBuf>` containing a possible root directory passed to the
/// `-c` arguement.
#[derive(Debug, Clone)]
pub struct Split {
    pub target: Target,
    pub path: Option<PathBuf>,
}

impl Split {
    pub fn new(target: Target, path: Option<PathBuf>) -> Split {
        Split { target, path }
    }
}

impl Command for Split {
    fn args(&self) -> Vec<&str> {
        let args: Vec<&str> = vec!["split-window", "-t", &self.target.combined];

        match self.path.as_ref().and_then(|p| p.to_str()) {
            Some(path) => [&args[..], &["-c", path]].concat(),
            None => args,
        }
    }
}

/// The Layout command calls select-layout with a specific pre-defined tmux
/// layout option see `tmux select-layout --help` for more options.
/// `target`: The target window. In the format `{session}:{window}.{paneIndex}`.
/// `layout`: The type of layout. ex `main-horizontal`.
#[derive(Debug, Clone)]
pub struct Layout {
    pub target: Target,
    pub layout: String,
}

impl Layout {
    pub fn new(target: Target, layout: String) -> Layout {
        Layout { target, layout }
    }
}

impl Command for Layout {
    fn args(&self) -> Vec<&str> {
        vec!["select-layout", "-t", &self.target.combined, &self.layout]
    }
}

/// A generic `SendKeys` command used to send "typed" commands to tmux. This is
/// used to initialize processes or tasks in specific window. Such as starting log
/// tails or running servers.
/// target: The target window. In the format `{session}:{window}.{paneIndex}`.
/// exec: The cli command to be run. ex. `tail -f logs/development.log`.
#[derive(Debug, Clone)]
pub struct SendKeys {
    pub target: Target,
    pub exec: String,
}

impl SendKeys {
    pub fn new(target: Target, exec: String) -> SendKeys {
        SendKeys { target, exec }
    }
}

impl Command for SendKeys {
    fn args(&self) -> Vec<&str> {
        vec![
            "send-keys",
            "-t",
            &self.target.combined,
            &self.exec,
            "KPEnter",
        ]
    }
}

/// Used to attach to the daemonized session.
/// name: The named session to attach too.
/// `path`: An `Option<PathBuf>` containing a possible root directory passed to the
/// `-c` arguement.
#[derive(Debug, Clone)]
pub struct Attach {
    pub target: Target,
    pub root_path: Option<PathBuf>,
}

impl Attach {
    pub fn new(target: Target, root_path: Option<PathBuf>) -> Attach {
        Attach { target, root_path }
    }
}

impl Command for Attach {
    fn call(&self) -> Result<Output, TmuxError> {
        if DEBUG.load() {
            println!("{:?}", &self.args());
        };

        tmux::attach(&self.args())
    }

    fn args(&self) -> Vec<&str> {
        let args: Vec<&str> = vec!["attach", "-t", &self.target.combined];

        let args = match self.root_path.as_ref().and_then(|p| p.to_str()) {
            Some(path) => [&args[..], &["-c", path]].concat(),
            None => args,
        };

        [&args[..], &[">/dev/null"]].concat()
    }
}

/// Used to move focus back to the first window.
/// target: The target window. In the format `{session}:{window}`.
#[derive(Debug, Clone)]
pub struct SelectWindow {
    pub target: Target,
}

impl SelectWindow {
    pub fn new(target: Target) -> SelectWindow {
        SelectWindow { target }
    }
}

impl Command for SelectWindow {
    fn args(&self) -> Vec<&str> {
        vec!["select-window", "-t", &self.target.combined]
    }
}

/// Used to move focus back to the top pane.
/// target: The target pane. In the format `{session}:{window}.{pane-target}`.
#[derive(Debug, Clone)]
pub struct SelectPane {
    pub target: Target,
}

impl SelectPane {
    pub fn new(target: Target) -> SelectPane {
        SelectPane { target }
    }
}

impl Command for SelectPane {
    fn args(&self) -> Vec<&str> {
        vec!["select-pane", "-t", &self.target.combined]
    }
}

/// Used to switch to a daemonized session when already within a tmux session.
/// name: The named session to switch to.
#[derive(Debug, Clone)]
pub struct SwitchClient {
    pub name: Target,
}

impl SwitchClient {
    pub fn new(name: &str) -> SwitchClient {
        SwitchClient {
            name: Target::new(name.to_string(), None, None),
        }
    }
}

impl Command for SwitchClient {
    fn args(&self) -> Vec<&str> {
        vec!["switch-client", "-t", &self.name.combined]
    }
}

/// Used for executing the `pre` option to execute commands before building the
/// tmux session.
/// exec: The command to execute
#[derive(Debug, Clone)]
pub struct Pre {
    pub exec: String,
}

impl Pre {
    pub fn new(exec: String) -> Pre {
        Pre { exec }
    }
}

impl Command for Pre {
    fn args(&self) -> Vec<&str> {
        // No-op!
        vec![]
    }

    fn call(&self) -> Result<Output, TmuxError> {
        if DEBUG.load() {
            println!("{:?}", &self.exec);
        };

        let cmd_array: Vec<&str> = self.exec.split(' ').collect();
        let (program, args) = cmd_array.split_first().ok_or(TmuxError::Pre)?;

        Ok(process::Command::new(program).args(args).output()?)
    }
}

/// The Command enum. Commands represent the series of commands sent to the
/// running tmux process to build a users env. This is an enum to support
/// containing all the commands that require running in a single Vec. This
/// allows a simple process of first in, first out command execution.
#[derive(Debug, Clone)]
pub enum Commands {
    Attach(Attach),
    Layout(Layout),
    Pre(Pre),
    SelectPane(SelectPane),
    SelectWindow(SelectWindow),
    SendKeys(SendKeys),
    Session(Session),
    Split(Split),
    SwitchClient(SwitchClient),
    Window(Window),
}

impl Commands {
    pub fn as_trait(&self) -> &dyn Command {
        match self {
            Commands::Attach(c) => c,
            Commands::Layout(c) => c,
            Commands::Pre(c) => c,
            Commands::SelectPane(c) => c,
            Commands::SelectWindow(c) => c,
            Commands::SendKeys(c) => c,
            Commands::Session(c) => c,
            Commands::Split(c) => c,
            Commands::SwitchClient(c) => c,
            Commands::Window(c) => c,
        }
    }
}

impl From<Attach> for Commands {
    fn from(command: Attach) -> Self {
        Commands::Attach(command)
    }
}

impl From<Layout> for Commands {
    fn from(command: Layout) -> Self {
        Commands::Layout(command)
    }
}

impl From<Pre> for Commands {
    fn from(command: Pre) -> Self {
        Commands::Pre(command)
    }
}

impl From<SelectPane> for Commands {
    fn from(command: SelectPane) -> Self {
        Commands::SelectPane(command)
    }
}

impl From<SelectWindow> for Commands {
    fn from(command: SelectWindow) -> Self {
        Commands::SelectWindow(command)
    }
}

impl From<SendKeys> for Commands {
    fn from(command: SendKeys) -> Self {
        Commands::SendKeys(command)
    }
}

impl From<Session> for Commands {
    fn from(command: Session) -> Self {
        Commands::Session(command)
    }
}

impl From<Split> for Commands {
    fn from(command: Split) -> Self {
        Commands::Split(command)
    }
}

impl From<SwitchClient> for Commands {
    fn from(command: SwitchClient) -> Self {
        Commands::SwitchClient(command)
    }
}

impl From<Window> for Commands {
    fn from(command: Window) -> Self {
        Commands::Window(command)
    }
}
