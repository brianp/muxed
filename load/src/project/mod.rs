//! The project module takes care of muxed related initialization. Locating the
//! users home directory. Finding the desired config files, and reading the
//! configs in.
pub mod parser;

use command::{Attach, Commands};
use common::project_paths::ProjectPaths;
use first_run::check_first_run;
use std::fs::File;
use std::io::prelude::*;
use tmux::has_session;
use yaml_rust::{Yaml, YamlLoader};

/// Using the provided project name, locate the path to that project file. It
/// should be something similar to: `~/.muxed/my_project.yml`
/// Read in the contents of the config (which should be Yaml), and parse the
/// contents as yaml.
///
/// # Examples
///
/// Given the project name "compiler" and a project file found at:
/// `~/.muxed/compiler.yml`.
///
/// ```
/// let yaml: Result<Vec<Yaml>, String> = read("compiler".to_string());
/// ```
///
/// `project_name`: The name of the project, corresponding to the project config
/// file.
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
        Some(Attach::new(&project_name, None).into())
    } else {
        None
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
}
