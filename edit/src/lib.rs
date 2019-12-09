//! Muxednew. A Muxed project Template Generator
extern crate common;
extern crate dirs;
extern crate libc;
#[cfg(test)]
extern crate rand;

use common::args::Args;

#[cfg(not(test))]
use dirs::home_dir;
use libc::system;
#[cfg(test)]
use rand::random;
use std::ffi::CString;
#[cfg(test)]
use std::fs;
use std::io;
use std::path::PathBuf;

static MUXED_FOLDER: &str = "muxed";

pub fn exec(args: Args) -> Result<(), io::Error> {
    let home = homedir().expect("Can't find home dir");
    let default_dir = format!("{}/.{}", home.display(), MUXED_FOLDER);
    let project_name = format!("{}.yml", &args.arg_project);
    let muxed_dir = match args.flag_p {
        Some(ref x) => x.as_str(),
        _ => default_dir.as_str(),
    };

    let command = format!("{} {}/{}", "$EDITOR", muxed_dir, project_name);
    let system_call = CString::new(command).unwrap();

    unsafe {
        system(system_call.as_ptr());
    };

    Ok(())
}

/// Return the users homedir as a string.
#[cfg(not(test))]
fn homedir() -> Result<PathBuf, String> {
    match home_dir() {
        Some(dir) => Ok(dir),
        None => Err(String::from("We couldn't find your home directory.")),
    }
}

/// Return the temp dir as the users home dir during testing.
#[cfg(test)]
fn homedir() -> Result<PathBuf, String> {
    Ok(PathBuf::from("/tmp"))
}
