//! The project module takes care of muxed related initialization. Locating the
/// users home directory. Finding the desired config files, and reading the
/// configs in.

use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::{YamlLoader, Yaml};
#[cfg(not(test))] use std::env::home_dir;
#[cfg(test)] use rand::random;
#[cfg(test)] use std::fs;

pub mod parser;
pub mod processor;

/// The muxed project folder name. Should be located in the users home dir as a
/// hidden directory.
static MUXED_FOLDER: &'static str = "muxed";

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
/// project_name: The name of the project, corresponding to the project config
/// file.
pub fn read(project_name: &String, project_dir: &Option<&str>) -> Result<Vec<Yaml>, String> {
    let home = try!(homedir().map_err(|e| e));
    let default_dir = format!("{}/.{}", home.display(), MUXED_FOLDER);
    let muxed_dir = project_dir.unwrap_or_else(|| default_dir.as_str());

    let config = format!("{}/{}.yml", muxed_dir, project_name);
    let path = Path::new(&config);

    let mut file = try!(File::open(path).map_err(|e| format!("No project configuration file was found with the name `{}`. Received error: {}", project_name, e.to_string())));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));

    let parsed_yaml = try!(YamlLoader::load_from_str(&contents).map_err(|e| e.to_string()));
    Ok(parsed_yaml)
}

/// Return the users homedir as a string.
#[cfg(not(test))] fn homedir() -> Result<PathBuf, String>{
    match home_dir() {
        Some(dir) => Ok(dir),
        None      => Err(String::from("We couldn't find your home directory."))
    }
}

/// Return the temp dir as the users home dir during testing.
#[cfg(test)] fn homedir() -> Result<PathBuf, String> {
    Ok(PathBuf::from("/tmp"))
}

#[test]
fn missing_file_returns_err() {
    let result = read(&String::from("not_a_file"), &None);
    assert!(result.is_err())
}

#[test]
fn poorly_formatted_file_returns_err() {
    let name = random::<u16>();
    let name1 = format!("/tmp/.muxed/{}.yml", name);
    let path = Path::new(&name1);
    let _ = fs::create_dir(Path::new("/tmp/.muxed/"));
    let mut buffer = File::create(path).unwrap();
    let _ = buffer.write(b"mix: [1,2,3]: muxed");

    let result = read(&format!("{}", name), &None);
    let _ = fs::remove_file(path);
    assert!(result.is_err());
}

#[test]
fn good_file_returns_ok() {
    let name = random::<u16>();
    let name1 = format!("/tmp/.muxed/{}.yml", name);
    let path = Path::new(&name1);
    let _ = fs::create_dir(Path::new("/tmp/.muxed/"));
    let mut buffer = File::create(path).unwrap();
    let _ = buffer.write(b"---
windows: ['cargo', 'vim', 'git']
");

    let result = read(&format!("{}", name), &None);
    let _ = fs::remove_file(path);
    assert!(result.is_ok());
}
