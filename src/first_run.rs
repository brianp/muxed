//! Muxednew. A Muxed project Template Generator
use std::fs::{create_dir};
use std::path::{Path};

pub fn check_first_run(muxed_dir: &str) {
    if !Path::new(muxed_dir).exists() {
        create_dir(muxed_dir).map_err(|e| format!("We noticed the configuration directory: `{}` didn't exist so we tried to create it, but something went wrong: {}", muxed_dir, e));
        println!("Looks like this is your first time here. Muxed could't find the configuration directory: `{}`", muxed_dir);
        println!("Creating that now \u{1F44C}\n")
    };
}
