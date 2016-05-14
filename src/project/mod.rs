//! The project module takes care of muxed related initialization. Locating the
/// users home directory. Finding the desired config files, and reading the
/// configs in.

use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::{YamlLoader, Yaml};
#[cfg(not(test))] use std::env::home_dir;

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
/// let yaml: Vec<Yaml> = read("compiler".to_string());
/// ```
///
/// project_name: The name of the project, corresponding to the project config
/// file.
pub fn read(project_name: &String) -> Vec<Yaml> {
    let config = format!("{}/.{}/{}.yml", homedir_string(), &MUXED_FOLDER, project_name);
    let path = Path::new(&config);
    let mut contents = String::new();
    let _ = File::open(path).expect("Config Read error").read_to_string(&mut contents);
    YamlLoader::load_from_str(&contents).expect("Yaml was not parsed")
}

/// Return the users homedir as a string.
#[cfg(not(test))] fn homedir_string() -> String {
    match home_dir() {
        Some(dir) => format!("{}", dir.display()),
        None      => panic!("Impossible to get your home dir!")
    }
}

/// Return the temp dir as the users home dir during testing.
#[cfg(test)] fn homedir_string() -> String {
    String::from("/tmp")
}
