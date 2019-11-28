//! Muxednew. A Muxed project Template Generator
extern crate common;
extern crate dirs;
#[cfg(test)]
extern crate rand;

use common::args::Args;
use common::first_run::check_first_run;

#[cfg(not(test))]
use dirs::home_dir;
#[cfg(test)]
use rand::random;
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::fs::File;
use std::fs::OpenOptions;
#[cfg(test)]
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

static TEMPLATE: &str = include_str!("template.yml");
static MUXED_FOLDER: &str = "muxed";

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
pub fn exec(args: Args) -> Result<(), String> {
    let home = homedir().expect("Can't find home dir");
    let default_dir = format!("{}/.{}", home.display(), MUXED_FOLDER);
    let project_name = format!("{}.yml", &args.arg_project);
    let muxed_dir = match args.flag_p {
        Some(ref x) => x.as_str(),
        _ => default_dir.as_str(),
    };

    check_first_run(&muxed_dir);

    let file = PathBuf::from(muxed_dir).join(&project_name);
    let template = modified_template(TEMPLATE, &file);
    write_template(&template, &file).unwrap();
    println!(
        "\u{270C} The template file {} has been written to {}\nHappy tmuxing!",
        project_name, muxed_dir
    );
    Ok(())
}

fn modified_template(template: &str, file: &PathBuf) -> String {
    template.replace("{file}", file.to_str().unwrap())
}

fn write_template(template: &str, path: &PathBuf) -> Result<(), String> {
    let path_str = path.to_str().unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("Could not create the file {}. Error: {}", &path_str, e))?;

    file.write_all(template.as_bytes()).map_err(|e| format!(
        "Could not write contents of template to the file {}. Error {}",
        &path_str, e
    ))?;

    file.sync_all()
        .map_err(|e| format!("Could not sync OS data post-write. Error: {}", e))?;

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

#[test]
fn expect_muxed_project_text() {
    let file = PathBuf::from("~/.muxed").join("superProject");
    let value = modified_template(TEMPLATE, &file);
    let result = value.contains("superProject");
    assert!(result);
}

#[test]
fn expect_muxed_dir_text() {
    let file = PathBuf::from("~/.muxed").join("superProject");
    let value = modified_template(TEMPLATE, &file);
    let result = value.contains("~/.muxed/");
    assert!(result);
}

#[test]
fn expect_no_file_name_placeholder() {
    let file = PathBuf::from("~/.my_dir").join("superProject");
    let value = modified_template(TEMPLATE, &file);
    let result = !value.contains("{file}");
    assert!(result);
}

#[test]
fn expect_project_name_with_dir() {
    let file = PathBuf::from("~/.my_dir").join("superProject.yml");
    let value = modified_template(TEMPLATE, &file);
    let result = value.contains("# ~/.my_dir/superProject.yml");
    assert!(result);
}

#[test]
fn expect_project_name_with_dir_and_trailing_slash() {
    let file = PathBuf::from("~/.my_dir/").join("superProject.yml");
    let value = modified_template(TEMPLATE, &file);
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
