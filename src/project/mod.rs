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
/// let yaml: Vec<Yaml> = main("compiler".to_string());
/// ```
///
/// project_name: The name of the project, corresponding to the project config
/// file.
pub fn main(project_name: &String) -> Vec<Yaml> {
    YamlLoader::load_from_str(&read(project_path(project_name))).unwrap()
}

/// Project path locates the full path of the project file as a String. It's in
/// a string format because I had issues passing values of `Path`. `Path` would be
/// a more appropriate type to return, alas we are here.
///
/// # Examples
///
/// ```
/// let path: String = project_path("compiler".to_string());
/// println!("{}", path);
/// => "/home/vagrant/.muxed/compiler.yml"
/// ```
///
/// project_name: The name of the project, corresponding to the project config
/// file.
fn project_path(project_name: &String) -> String {
    format!("{}/.{}/{}.yml", homedir_string(), &MUXED_FOLDER, project_name)
}

/// Read takes in the string path for a config file and returns the contents of
/// the file. Again we'de rather have a `Path` passed in to the function but we
/// I had issues with passing `Path` references or values.
///
/// config_str: The string path to the config file.
fn read(config_str: String) -> String {
    let path = Path::new(&config_str);
    let mut s = String::new();
    let _ = File::open(path).expect("Config Read error").read_to_string(&mut s);

    return s
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

#[test]
pub fn project_path_returns_muxed_in_homedir() {
    let path = format!("{}", project_path(&"test".to_string()));
    let new  = format!("{}", Path::new("/tmp/.muxed/test.yml").display());
    assert_eq!(path, new)
}
