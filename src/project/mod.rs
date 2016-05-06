use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::{YamlLoader, Yaml};
#[cfg(not(test))] use std::env::home_dir;

pub mod parser;
pub mod processor;

static MUXED_FOLDER: &'static str = "muxed";

pub fn main(project_name: &String) -> Vec<Yaml> {
    YamlLoader::load_from_str(&read(project_path(project_name))).unwrap()
}

pub fn project_path(project_name: &String) -> String {
    format!("{}/.{}/{}.yml", homedir_string(), &MUXED_FOLDER.to_string(), project_name)
}

fn read(config_str: String) -> String {
    let path = Path::new(&config_str);
    let mut s = String::new();
    let _ = File::open(path).expect("Config Read error").read_to_string(&mut s);

    return s
}

#[cfg(not(test))] fn homedir_string() -> String {
    match home_dir() {
        Some(dir) => format!("{}", dir.display()),
        None      => panic!("Impossible to get your home dir!")
    }
}

#[cfg(test)] fn homedir_string() -> String {
    String::from("/tmp")
}

#[test]
pub fn project_path_returns_muxed_in_homedir() {
    let path = format!("{}", project_path(&"test".to_string()));
    let new  = format!("{}", Path::new("/tmp/.muxed/test.yml").display());
    assert_eq!(path, new)
}
