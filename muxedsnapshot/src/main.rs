//! Muxedsnapshot. A tmux session cloner for Muxed.
#![feature(proc_macro)]
extern crate clap;
extern crate libc;
extern crate regex;
extern crate yaml_rust;
extern crate serde;
extern crate serde_yaml;
#[cfg(test)] extern crate rand;

#[macro_use]
extern crate serde_derive;

mod tmux;
mod capture;

use clap::{Arg, App, AppSettings};
use std::process::exit;
use std::path::{Path, PathBuf};
use std::fs::{OpenOptions, create_dir};
use std::io::Write;
#[cfg(not(test))] use std::env::home_dir;

#[macro_export]
macro_rules! try_or_err (
    ($expr: expr) => ({
        match $expr {
            Ok(val) => val,
            Err(e) => {
              println!("Muxedsnapshot ran in to a problem:");
              println!("{}", e);
              exit(1);
            }
        }
    })
);

static MUXED_FOLDER: &'static str = "muxed";

/// The main execution method.
/// Accepts two arguments. -n for the name of the project file and -t to target
/// the session.
///
/// # Examples
///
/// You can run the command:
///
/// ```
/// $ ./muxedsnapshot -n my_new_project -t existing_sesion
/// ```
///
/// or
///
/// ```
/// $ ./muxed snapshot -n jasper -t 1
/// ```
pub fn main() {
    let matches = App::new("Muxedsnapshot")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Brian Pearce")
                      .about("A TMUX session codifier for Muxed")
                      .setting(AppSettings::TrailingVarArg)
                      .arg(Arg::with_name("NEW_PROJECT_NAME")
                           .short("n")
                           .help("The name of your new project file to create")
                           .multiple(false)
                           .required(true)
                           .takes_value(true))
                      .arg(Arg::with_name("SESSION")
                           .short("t")
                           .multiple(false)
                           .required(true)
                           .takes_value(true)
                           .help("The name of the TMUX session to codify"))
                      .arg(Arg::with_name("PROJECT_DIR")
                           .short("p")
                           .multiple(false)
                           .value_name("PROJECT_DIR")
                           .takes_value(true)
                           .help("The directory your project config files should live in. Defaults to ~/.muxed/"))
                      .arg(Arg::with_name("force")
                           .short("f")
                           .long("force")
                           .multiple(false)
                           .required(false)
                           .takes_value(false)
                           .help("Overwrite existing file if one exists"))
                      .get_matches();

    let home = try_or_err!(homedir().map_err(|e| e));
    let default_dir = format!("{}/.{}", home.display(), MUXED_FOLDER);
    let project_name = format!("{}.yml", matches.value_of("NEW_PROJECT_NAME").unwrap());
    let session_name = matches.value_of("SESSION").unwrap();
    let muxed_dir = matches.value_of("PROJECT_DIR").unwrap_or_else(|| default_dir.as_str());
    let new_project_path = PathBuf::from(format!("{}/{}", muxed_dir, project_name));

    if !Path::new(muxed_dir).exists() {
        try_or_err!(create_dir(muxed_dir).map_err(|e| format!("We noticed the configuration directory: `{}` didn't exist so we tried to create it, but something went wrong: {}", muxed_dir, e)));
        println!("Looks like this is your first time here. Muxed could't find the configuration directory: `{}`", muxed_dir);
        println!("Creating that now \u{1F44C}\n")
    };

    let session = try_or_err!(tmux::inspect(&session_name));
    let s = serde_yaml::to_string(&session).unwrap();

    try_or_err!(write_config(s, &new_project_path, matches.is_present("force")));
    println!("We made a snapshot of your session! \u{1F60A}")
}

/// Write the new file
fn write_config<S>(template: S, path: &PathBuf, force: bool) -> Result<(), String>
    where S: Into<String>
{
    let path_str = path.to_str().unwrap();
    let mut file = try!(OpenOptions::new()
                            .write(true)
                            .truncate(force)
                            .create(force)
                            .create_new(!force)
                            .open(path)
                            .map_err(|e| format!("Could not create the file {}. Error: {}", &path_str, e)));
    try!(file.write_all(template.into().as_bytes()).map_err(|e| format!("Could not write contents of template to the file {}. Error {}", &path_str, e)));
    try!(file.sync_all().map_err(|e| format!("Could not sync OS data post-write. Error: {}", e)));
    Ok(())
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

#[cfg(test)]
mod test {
    use super::write_config;
    use std::path::PathBuf;
    use std::io::{Read, Write};
    use std::fs;
    use std::fs::File;
    use rand::random;

    #[test]
    fn expect_ok_result() {
        let name = random::<u16>();
        let name1 = format!("/tmp/{}.yml", name);
        let path = PathBuf::from(name1);
        let result = write_config("test template", &path, false);
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }

    #[test]
    fn expect_err_result() {
        let name = random::<u16>();
        let name1 = format!("/tmp/non_existent_path/{}.yml", name);
        let path = PathBuf::from(name1);
        let result = write_config("test template", &path, false);
        assert!(result.is_err());
    }

    #[test]
    fn expect_file_to_exist() {
        let name = random::<u16>();
        let name1 = format!("/tmp/{}.yml", name);
        let path = PathBuf::from(name1);
        let _ = write_config("test template", &path, false);
        let result = &path.exists();
        let _ = fs::remove_file(&path);
        assert!(result);
    }

    #[test]
    fn expect_file_not_to_exist() {
        let name = random::<u16>();
        let name1 = format!("/tmp/non_existent_path/{}.yml", name);
        let path = PathBuf::from(name1);
        let _ = write_config("test template", &path, false);
        assert!(!path.exists());
    }

    #[test]
    fn expect_no_truncation_or_overwrite() {
        // Write a file with content
        let name = random::<u16>();
        let name1 = format!("/tmp/{}.yml", name);
        let path = PathBuf::from(&name1);
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");

        // Attempt to create the same named file with new content
        let _ = write_config("new_content", &path, false);

        // Read the file content
        let mut f = File::open(&path).unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        assert_eq!(s, "original content");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_err_when_file_exists() {
        let name = random::<u16>();
        let name1 = format!("/tmp/{}.yml", name);
        let path = PathBuf::from(&name1);
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let result = write_config("new_content", &path, false);

        assert!(result.is_err());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_ok_when_file_exists_using_force() {
        let name = random::<u16>();
        let name1 = format!("/tmp/{}.yml", name);
        let path = PathBuf::from(&name1);
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let result = write_config("new_content", &path, true);

        assert!(result.is_ok());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_truncation_or_overwrite_using_force() {
        // Write a file with content
        let name = random::<u16>();
        let name1 = format!("/tmp/{}.yml", name);
        let path = PathBuf::from(&name1);
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");

        // Attempt to create the same named file with new content
        let _ = write_config("new content", &path, true);

        // Read the file content
        let mut f = File::open(&path).unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        assert_eq!(s, "new content");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_file_not_to_exist_using_force() {
        let name = random::<u16>();
        let name1 = format!("/tmp/non_existent_path/{}.yml", name);
        let path = PathBuf::from(name1);
        let _ = write_config("test template", &path, true);
        assert!(!path.exists());
    }

    #[test]
    fn expect_ok_result_using_force_when_file_doesnt_exist() {
        let name = random::<u16>();
        let name1 = format!("/tmp/{}.yml", name);
        let path = PathBuf::from(name1);
        let result = write_config("test template", &path, true);
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }
}
