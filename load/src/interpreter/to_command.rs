use crate::command::{
    Attach, Commands, Layout, Pre, SelectPane, SelectWindow, SendKeys, Split, Window,
};
use crate::interpreter::error::InterpreterError;
use common::tmux::Target;
use common::tmux::session::{NodeRef, Session};

type Result<T> = std::result::Result<T, InterpreterError>;

pub(crate) struct PlanContext<'a> {
    first: bool,
    session: &'a Session,
}

/// The `Plan` trait defines an interface for types that can generate
/// an ordered sequence of commands representing a tmux session configuration.
///
/// Implementors of this trait can be "planned," meaning their structure and settings
/// can be translated into a concrete list of commands to drive tmux programmatically.
/// For example, the `Session` struct implements `Plan` to output a complete step-by-step
/// collection of tmux commands (windows, panes, pre/post hooks, attachments, selections, etc).
///
/// # Example (for `Session`)
///
/// Calling `command_plan()` on a `Session` will:
///   - Insert any global pre-commands
///   - Iterate over windows and panes,
///     translating each into the commands required to create them in tmux
///   - Select the appropriate active window and pane
///   - If not in daemonized mode, add the command to attach to the session
///
/// # Returns
///
/// Returns a `Result<Vec<Commands>>`:
///   - `Ok(commands)` on success, where `commands` is the full sequence to establish the session in tmux.
///   - `Err(InterpreterError)` on failure (such as invalid configuration).
///
/// This trait is most commonly used to turn high-level session specifications into executable tmux command sequences
/// for scripting or API invocation.
pub trait Plan {
    fn command_plan(&self) -> Result<Vec<Commands>>;
}

impl Plan for Session {
    fn command_plan(&self) -> Result<Vec<Commands>> {
        let mut commands: Vec<Commands> = vec![];

        if let Some(pre) = self.pre() {
            commands.extend(pre.iter().map(|cmd| Pre::new(cmd.clone()).into()));
        }

        let mut active_target: Option<Target> = None;

        for node in self.iter() {
            match node {
                NodeRef::Window { window, index } => {
                    if let (true, None) = (window.active, &active_target) {
                        active_target = window.target.clone();
                    }

                    let first = index == 0;

                    let ctx = PlanContext {
                        first,
                        session: self,
                    };
                    commands.extend(window.to_commands(ctx)?);
                }
                // Add NodeRef::Pane logic as needed
                NodeRef::Pane { pane, .. } => {
                    let ctx = PlanContext {
                        first: false,
                        session: self,
                    };
                    commands.extend(pane.to_commands(ctx)?);
                }
            }
        }

        let window_index = self.config.as_ref().map(|c| c.base_index).unwrap_or(0);
        let pane_index = self.config.as_ref().map(|c| c.pane_base_index).unwrap_or(0);

        // Only the first active window found will be selected as active
        if let Some(target) = active_target {
            let pane = target.extend(pane_index)?;
            commands.push(SelectWindow::new(target).into());
            commands.push(SelectPane::new(pane).into());
        } else {
            let session = self
                .target
                .clone()
                .ok_or(InterpreterError::SessionTargetRequired)?;
            let window = session.extend(window_index)?;
            let pane = window.extend(pane_index)?;
            commands.push(SelectWindow::new(window).into());
            commands.push(SelectPane::new(pane).into());
        }

        if self.daemonize.is_none() {
            let target = match self.target.clone() {
                Some(target) => target,
                None => {
                    let name = self
                        .name
                        .clone()
                        .ok_or(InterpreterError::SessionTargetRequired)?;
                    Target::new(name, None, None)
                }
            };
            commands.push(Attach::new(target, self.root.clone()).into());
        }

        Ok(commands)
    }
}

/// The `ToCommand` trait defines an interface for generating tmux commands
/// from specific entities (like windows or panes) in the session specification.
/// Implementations produce a list of `Commands` required to realize the described
/// window or pane within tmux, using context from planning.
pub trait ToCommand {
    fn to_commands(&self, ctx: PlanContext) -> Result<Vec<Commands>>;
}

/// Implementation of `ToCommand` for a tmux `Window`.
///
/// This method returns a sequence of commands to set up a tmux window,
/// including creating the window/session, optionally changing path,
/// running pre-window commands, adding splits for panes, and sending
/// an initial command if specified.
impl ToCommand for common::tmux::Window {
    fn to_commands(&self, ctx: PlanContext) -> Result<Vec<Commands>> {
        let mut commands = vec![];

        let session_name = ctx
            .session
            .name
            .as_ref()
            .ok_or(InterpreterError::SessionNameRequired)?;
        let target = self
            .target
            .clone()
            .ok_or(InterpreterError::WindowTargetRequired)?;

        if ctx.first {
            commands.push(
                crate::command::Session::new(session_name, &self.name, ctx.session.root.clone())
                    .into(),
            );
        } else {
            commands.push(Window::new(&self.name, target.clone(), self.path.clone()).into());
        }

        // Navigate to the path
        if let Some(path) = self.path.as_ref() {
            commands.push(SendKeys::new(target.clone(), format!("cd {}", path.display())).into());
        }

        if let Some(pre) = ctx.session.pre_window.as_ref() {
            for cmd in pre.iter() {
                commands.push(SendKeys::new(target.clone(), cmd.clone()).into());
            }
        }

        for _ in 0..self.panes.len().saturating_sub(1) {
            commands.push(Split::new(target.clone(), self.path.clone()).into());
        }

        if let Some(layout) = self.layout.as_ref() {
            commands.push(Layout::new(target.clone(), layout.clone()).into());
        }

        // Send the command
        if let Some(command) = self.command.as_ref() {
            commands.push(SendKeys::new(target.clone(), command.clone()).into());
        }

        Ok(commands)
    }
}

/// Implementation of `ToCommand` for a tmux `Pane`.
///
/// This method assembles a list of commands to realize a pane, typically
/// only sending pre-window commands and the pane's custom command (if present).
impl ToCommand for common::tmux::Pane {
    fn to_commands(&self, ctx: PlanContext) -> Result<Vec<Commands>> {
        let mut commands: Vec<Commands> = vec![];

        let target = self
            .target
            .clone()
            .ok_or(InterpreterError::PaneTargetRequired)?;

        if let Some(pre) = ctx.session.pre_window.as_ref() {
            for cmd in pre.iter() {
                commands.push(SendKeys::new(target.clone(), cmd.clone()).into());
            }
        }

        if let Some(cmd) = self.command.as_ref() {
            commands.push(SendKeys::new(target.clone(), cmd.clone()).into());
        };

        if self.active {
            commands.push(SelectPane::new(target).into());
        };

        Ok(commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::tmux::{Config, Pane, Session, Target, Window};

    fn basic_config() -> Config {
        Config {
            base_index: 0,
            pane_base_index: 0,
        }
    }

    #[test]
    fn expect_1_session_plan() {
        let session = Session {
            name: Some("muxed".into()),
            target: Some(Target::new("muxed", None, None)),
            windows: vec![
                Window {
                    name: "cargo".into(),
                    target: Some(Target::new("muxed", Some(0), None)),
                    panes: vec![],
                    ..Default::default()
                },
                Window {
                    name: "vim".into(),
                    target: Some(Target::new("muxed", Some(1), None)),
                    panes: vec![],
                    ..Default::default()
                },
                Window {
                    name: "git".into(),
                    target: Some(Target::new("muxed", Some(2), None)),
                    panes: vec![],
                    ..Default::default()
                },
            ],
            config: Some(basic_config()),
            ..Default::default()
        };
        let commands = session.command_plan().unwrap();
        let remains: Vec<_> = commands
            .iter()
            .filter(|x| matches!(x, Commands::Session(_)))
            .collect();
        assert_eq!(remains.len(), 1);
    }

    #[test]
    fn expect_2_windows_from_session_plan() {
        let session = Session {
            name: Some("muxed".into()),
            target: Some(Target::new("muxed", None, None)),
            windows: vec![
                Window {
                    name: "cargo".into(),
                    target: Some(Target::new("muxed", Some(0), None)),
                    panes: vec![],
                    ..Default::default()
                },
                Window {
                    name: "vim".into(),
                    target: Some(Target::new("muxed", Some(1), None)),
                    panes: vec![],
                    ..Default::default()
                },
            ],
            config: Some(basic_config()),
            ..Default::default()
        };
        let commands = session.command_plan().unwrap();
        let remains: Vec<_> = commands
            .iter()
            .filter(|x| matches!(x, Commands::Window(_)))
            .collect();
        assert_eq!(remains.len(), 1);
    }

    #[test]
    fn expect_1_attach_command() {
        let session = Session {
            name: Some("muxed".into()),
            target: Some(Target::new("muxed", None, None)),
            windows: vec![Window {
                name: "cargo".into(),
                target: Some(Target::new("muxed", Some(0), None)),
                panes: vec![],
                ..Default::default()
            }],
            config: Some(basic_config()),
            ..Default::default()
        };
        let commands = session.command_plan().unwrap();
        let remains: Vec<_> = commands
            .iter()
            .filter(|x| matches!(x, Commands::Attach(_)))
            .collect();
        assert_eq!(remains.len(), 1);
    }

    #[test]
    fn expect_1_split_window_command() {
        let session = Session {
            name: Some("muxed".into()),
            target: Some(Target::new("muxed", None, None)),
            windows: vec![Window {
                name: "editor".into(),
                target: Some(Target::new("muxed", Some(0), None)),
                panes: vec![
                    Pane {
                        command: Some("vim".into()),
                        target: Some(Target::new("muxed", Some(0), Some(0))),
                        ..Default::default()
                    },
                    Pane {
                        command: Some("guard".into()),
                        target: Some(Target::new("muxed", Some(0), Some(1))),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            config: Some(basic_config()),
            ..Default::default()
        };
        let commands = session.command_plan().unwrap();
        let remains: Vec<_> = commands
            .iter()
            .filter(|x| matches!(x, Commands::Split(_)))
            .collect();
        assert_eq!(remains.len(), 1);
    }

    #[test]
    fn expect_no_send_keys_with_empty_panes_syscommands() {
        let session = Session {
            name: Some("muxed".into()),
            target: Some(Target::new("muxed", None, None)),
            windows: vec![Window {
                name: "editor".into(),
                target: Some(Target::new("muxed", Some(0), None)),
                panes: vec![Pane {
                    command: None,
                    target: Some(Target::new("muxed", Some(0), Some(0))),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            config: Some(basic_config()),
            ..Default::default()
        };
        let commands = session.command_plan().unwrap();
        let remains: Vec<_> = commands
            .iter()
            .filter(|x| matches!(x, Commands::SendKeys(_)))
            .collect();
        assert_eq!(remains.len(), 0);
    }

    #[test]
    fn expect_1_layout_command() {
        let window = Window {
            name: "editor".into(),
            target: Some(Target::new("muxed", Some(0), None)),
            panes: vec![
                Pane {
                    command: Some("vim".into()),
                    target: Some(Target::new("muxed", Some(0), Some(0))),
                    ..Default::default()
                },
                Pane {
                    command: Some("guard".into()),
                    target: Some(Target::new("muxed", Some(0), Some(1))),
                    ..Default::default()
                },
            ],
            layout: Some("main-vertical".into()),
            ..Default::default()
        };

        let session = Session {
            name: Some("muxed".into()),
            target: Some(Target::new("muxed", None, None)),
            windows: vec![window],
            config: Some(basic_config()),
            ..Default::default()
        };

        let commands = session.command_plan().unwrap();
        let remains: Vec<_> = commands
            .iter()
            .filter(|x| matches!(x, Commands::Layout(_)))
            .collect();
        assert_eq!(remains.len(), 1);
    }

    #[test]
    fn expect_no_layout_for_window_without_layout() {
        let window = Window {
            name: "editor".into(),
            target: Some(Target::new("muxed", Some(0), None)),
            panes: vec![
                Pane {
                    command: Some("vim".into()),
                    target: Some(Target::new("muxed", Some(0), Some(0))),
                    ..Default::default()
                },
                Pane {
                    command: Some("guard".into()),
                    target: Some(Target::new("muxed", Some(0), Some(1))),
                    ..Default::default()
                },
            ],
            layout: None,
            ..Default::default()
        };

        let session = Session {
            name: Some("muxed".into()),
            target: Some(Target::new("muxed", None, None)),
            windows: vec![window],
            config: Some(basic_config()),
            ..Default::default()
        };
        let commands = session.command_plan().unwrap();
        let remains: Vec<_> = commands
            .iter()
            .filter(|x| matches!(x, Commands::Layout(_)))
            .collect();
        assert_eq!(remains.len(), 0);
    }
}
