//! Muxednew. A Muxed project Template Generator
extern crate clap;
#[cfg(test)] extern crate rand;

use clap::{Arg, App};
use std::path::{Path, PathBuf};
use std::fs::{OpenOptions, create_dir};
use std::io::Write;
use std::process::exit;
#[cfg(test)] use rand::random;
#[cfg(test)] use std::fs;
#[cfg(test)] use std::fs::File;
#[cfg(test)] use std::io::Read;
#[cfg(not(test))] use std::env::home_dir;

#[macro_export]
macro_rules! try_or_err (
    ($expr: expr) => ({
        match $expr {
            Ok(val) => val,
            Err(e) => {
              println!("Muxednew ran in to a problem:");
              println!("{}", e);
              exit(1)
            }
        }
    })
);

static TEMPLATE: &'static str = include_str!("template.yml");
static MUXED_FOLDER: &'static str = "muxed";

/// The main execution method.
/// Accept the name of a project to create a configuration file in the
/// `~/.muxed/` directory.
///
/// # Examples
///
/// You can run the command:
///
/// ```
/// $ ./muxednew projectName
/// ```
///
/// or specify the directory target of the file:
///
/// ```
/// $ ./muxednew -p ~/.some_other_dir/ projectName
/// ```
pub fn main() {
    let matches = App::new("Muxednew")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Brian Pearce")
                      .about("A Muxed project template generator")
                      .arg(Arg::with_name("PROJECT_NAME")
                           .help("The name of your project to create")
                           .index(1)
                           .required(true)
                           .takes_value(true))
                      .arg(Arg::with_name("PROJECT_DIR")
                           .short("-p")
                           .multiple(false)
                           .value_name("PROJECT_DIR")
                           .takes_value(true)
                           .help("The directory your project config files live in. Defaults to ~/.muxed/"))
                      .get_matches();

    let home = try_or_err!(homedir().map_err(|e| e));
    let default_dir = format!("{}/.{}", home.display(), MUXED_FOLDER);
    let project_name = format!("{}.yml", matches.value_of("PROJECT_NAME").unwrap());
    let muxed_dir = matches.value_of("PROJECT_DIR").unwrap_or_else(|| default_dir.as_str());

    if !Path::new(muxed_dir).exists() {
        try_or_err!(create_dir(muxed_dir).map_err(|e| format!("We noticed the configuration directory: `{}` didn't exist so we tried to create it, but something went wrong: {}", muxed_dir, e)));
        println!("Looks like this is your first time here. Muxed could't find the configuration directory: `{}`", muxed_dir);
        println!("Creating that now \u{1F44C}\n")
    };

    let file = PathBuf::from(muxed_dir).join(&project_name);
    let template = modified_template(TEMPLATE, &file);
    try_or_err!(write_template(&template, &file));
    println!("\u{270C} The template file {} has been written to {}\nHappy tmuxing!", project_name, muxed_dir);
}

fn modified_template(template: &str, file: &PathBuf) -> String {
    template.replace("{file}", file.to_str().unwrap())
}

fn write_template(template: &String, path: &PathBuf) -> Result<(), String> {
    let path_str = path.to_str().unwrap();
    let mut file = try!(OpenOptions::new().write(true)
                            .create_new(true)
                            .open(path)
                            .map_err(|e| format!("Could not create the file {}. Error: {}", &path_str, e)));
    try!(file.write_all(template.as_bytes()).map_err(|e| format!("Could not write contents of template to the file {}. Error {}", &path_str, e)));
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

#[test]
fn expect_muxed_project_text() {
    let file = PathBuf::from("~/.muxed").join("superProject");
    let value  = modified_template(TEMPLATE, &file);
    let result = value.contains("superProject");
    assert!(result);
}

#[test]
fn expect_muxed_dir_text() {
    let file = PathBuf::from("~/.muxed").join("superProject");
    let value  = modified_template(TEMPLATE, &file);
    let result = value.contains("~/.muxed/");
    assert!(result);
}

#[test]
fn expect_no_file_name_placeholder() {
    let file = PathBuf::from("~/.my_dir").join("superProject");
    let value  = modified_template(TEMPLATE, &file);
    let result = !value.contains("{file}");
    assert!(result);
}

#[test]
fn expect_project_name_with_dir() {
    let file = PathBuf::from("~/.my_dir").join("superProject.yml");
    let value  = modified_template(TEMPLATE, &file);
    let result = value.contains("# ~/.my_dir/superProject.yml");
    assert!(result);
}

#[test]
fn expect_project_name_with_dir_and_trailing_slash() {
    let file = PathBuf::from("~/.my_dir/").join("superProject.yml");
    let value  = modified_template(TEMPLATE, &file);
    let result = value.contains("# ~/.my_dir/superProject.yml");
    assert!(result);
}

#[test]
fn expect_ok_result() {
    let name = random::<u16>();
    let name1 = format!("/tmp/{}.yml", name);
    let path = PathBuf::from(name1);
    let result = write_template(&"test template".to_string(), &path);
    let _ = fs::remove_file(path);
    assert!(result.is_ok());
}

#[test]
fn expect_err_result() {
    let name = random::<u16>();
    let name1 = format!("/tmp/non_existent_path/{}.yml", name);
    let path = PathBuf::from(name1);
    let result = write_template(&"test template".to_string(), &path);
    assert!(result.is_err());
}

#[test]
fn expect_file_to_exist() {
    let name = random::<u16>();
    let name1 = format!("/tmp/{}.yml", name);
    let path = PathBuf::from(name1);
    let _ = write_template(&"test template".to_string(), &path);
    let result = &path.exists();
    let _ = fs::remove_file(&path);
    assert!(result);
}

#[test]
fn expect_file_not_to_exist() {
    let name = random::<u16>();
    let name1 = format!("/tmp/non_existent_path/{}.yml", name);
    let path = PathBuf::from(name1);
    let _ = write_template(&"test template".to_string(), &path);
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
    let _ = write_template(&"new_content".to_string(), &path);

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
    let result = write_template(&"new_content".to_string(), &path);

    assert!(result.is_err());
    let _ = fs::remove_file(&path);
}
