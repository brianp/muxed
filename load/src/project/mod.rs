//! The project module takes care of muxed related initialization. Locating the
//! users home directory. Finding the desired config files, and reading the
//! configs in.
pub mod parser;

use command::{Attach, Commands, SwitchClient};
use common::project_paths::ProjectPaths;
use common::tmux::has_session;
use first_run::check_first_run;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{Yaml, YamlLoader};

static TMUX_ENV_VAR: &str = "TMUX";

/// Using the provided project name, locate the path to that project file. It
/// should be something similar to: `~/.muxed/my_project.yml`
/// Read in the contents of the config (which should be Yaml), and parse the
/// contents as yaml.
///
/// `project_name`: The name of the project, corresponding to the project config
/// file.
/// `project_paths`: The struct of paths
///
/// # Examples
///
/// Given the project name "compiler" and a project file found at:
/// `~/.muxed/compiler.yml`.
///
/// ```rust,no_run
/// extern crate common;
/// extern crate load;
/// extern crate yaml_rust;
///
/// use common::project_paths::ProjectPaths;
/// use load::project::read;
/// use std::path::PathBuf;
/// use yaml_rust::{Yaml, YamlLoader};
///
/// let paths = ProjectPaths::new(
///     PathBuf::from("/tmp"),
///     PathBuf::from("/tmp/.muxed"),
///     PathBuf::from("/tmp/.muxed/projectname.yml")
/// );
///
/// let yaml: Result<Vec<Yaml>, String> = read("compiler", &paths);
///
/// assert!(yaml.is_ok());
/// ```
pub fn read(project_name: &str, project_paths: &ProjectPaths) -> Result<Vec<Yaml>, String> {
    check_first_run(&project_paths.project_directory)?;

    let mut file = File::open(&project_paths.project_file).map_err(|e| format!("No project configuration file was found with the name `{}` in the directory `{}`. Received error: {}", project_name, &project_paths.project_directory.display(), e.to_string()))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;

    let parsed_yaml = YamlLoader::load_from_str(&contents).map_err(|e| e.to_string())?;

    Ok(parsed_yaml)
}

/// Find out if a tmux session is already active with this name. If it is active
/// return `Some<Commands::Attach>` with a command to attach to the session. If a
/// session is not active return None and let the app carry on.
pub fn session_exists(project_name: &str) -> Option<Commands> {
    if has_session(project_name).success() {
        Some(open(project_name))
    } else {
        None
    }
}

/// Check to see how we want to open the project. Do we need to attach to a new
/// tmux session or can we switch the client from a running session.
///
/// # Examples
///
/// ```rust
/// extern crate load;
///
/// use load::command::{Attach, Commands, Command};
/// use load::project::open;
///
/// let correct_type = match open("muxed") {
///     Commands::Attach(_) => true,
///     _ => false,
/// };
///
/// assert!(correct_type)
/// ```
pub fn open(project_name: &str) -> Commands {
    if env::var_os(TMUX_ENV_VAR).is_some() {
        SwitchClient::new(&project_name).into()
    } else {
        Attach::new(&project_name, None).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::rand_names;
    use std::fs;

    #[test]
    fn missing_file_returns_err() {
        let project_paths = ProjectPaths::from_strs("/tmp", ".muxed", "");
        let result = read(&String::from("not_a_file"), &project_paths);
        assert!(result.is_err())
    }

    #[test]
    fn poorly_formatted_file_returns_err() {
        let name = rand_names::project_file_name();
        let project_paths = ProjectPaths::from_strs("/tmp", ".muxed", &name);

        let _ = fs::create_dir(&project_paths.project_directory);
        let mut buffer = File::create(&project_paths.project_file).unwrap();
        let _ = buffer.write(b"mix: [1,2,3]: muxed");
        let _ = buffer.sync_all();

        let result = read(&name, &project_paths);
        let _ = fs::remove_file(&project_paths.project_file);
        assert!(result.is_err());
    }

    #[test]
    fn good_file_returns_ok() {
        let name = rand_names::project_file_name();
        let project_paths = ProjectPaths::from_strs("/tmp", ".muxed", &name);

        let _ = fs::create_dir(&project_paths.project_directory);
        let mut buffer = File::create(&project_paths.project_file).unwrap();
        let _ = buffer.write(
            b"---
    windows: ['cargo', 'vim', 'git']
    ",
        );
        let _ = buffer.sync_all();

        let result = read(&name, &project_paths);
        let _ = fs::remove_file(&project_paths.project_file);
        assert!(result.is_ok());
    }

    #[test]
    fn open_returns_attach_in_bare_context() {
        let attach_command = match open("muxed") {
            Commands::Attach(_) => true,
            _ => false,
        };

        assert!(attach_command);
    }

    #[test]
    fn open_returns_switch_client_in_nested_context() {
        let _ = env::set_var(TMUX_ENV_VAR, "somestring");
        let switch_command = match open("muxed") {
            Commands::SwitchClient(_) => true,
            _ => false,
        };
        let _ = env::remove_var(TMUX_ENV_VAR);

        assert!(switch_command);
    }
}
