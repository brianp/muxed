use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
#[cfg(not(test))] use std::env::home_dir;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};

static MUXED_FOLDER: &'static str = "muxed";

pub fn open(project_name: String) -> Vec<Yaml> {
    load_yaml(&read(&format!("./{}{}", MUXED_FOLDER, project_name)))
}

#[cfg(not(test))] fn homedir_string() -> String {
    match home_dir() {
        Some(dir) => format!("{}", dir.display()),
        None      => panic!("Impossible to get your home dir!")
    }
}

fn parse_config() {
}

fn read(config_str: &String) -> String {
    let path = Path::new(config_str);
    let mut s = String::new();
    File::open(path).expect("Config Read error").read_to_string(&mut s);

    return s
}

fn load_yaml(yaml_string: &String) -> Vec<Yaml> {
    return YamlLoader::load_from_str(yaml_string).unwrap();
}
