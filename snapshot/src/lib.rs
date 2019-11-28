//! Muxedsnapshot. A tmux session cloner for Muxed.
extern crate common;
extern crate dirs;
extern crate regex;
extern crate serde;
extern crate serde_yaml;
#[cfg(test)]
extern crate rand;

pub mod capture;
pub mod tmux;

#[macro_use]
extern crate serde_derive;

use common::args::Args;
use common::first_run::check_first_run;
#[cfg(not(test))]
use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

static MUXED_FOLDER: &str = "muxed";

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
pub fn exec(args: Args) -> Result<(), String> {
    let home = homedir().expect("Can't find home dir");
    let default_dir = format!("{}/.{}", home.display(), MUXED_FOLDER);
    let project_name = format!("{}.yml", &args.arg_project);
    let session_name = &args.flag_t.expect("No TMUX session running");
    let muxed_dir = match args.flag_p {
        Some(ref x) => x.as_str(),
        _ => default_dir.as_str(),
    };
    let new_project_path = PathBuf::from(format!("{}/{}", muxed_dir, project_name));

    check_first_run(&muxed_dir);

    let session = tmux::inspect(&session_name).unwrap();
    let s = serde_yaml::to_string(&session).unwrap();

    write_config(s, &new_project_path, args.flag_f).unwrap();
    println!("We made a snapshot of your session! \u{1F60A}");
    Ok(())
}

/// Write the new file
fn write_config<S>(template: S, path: &PathBuf, force: bool) -> Result<(), String>
where
    S: Into<String>,
{
    let path_str = path.to_str().expect("Path could not be opened");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(force)
        .create(force)
        .create_new(!force)
        .open(path)
        .map_err(|e| format!("Could not create the file {}. Error: {}", &path_str, e))?;

    file.write_all(template.into().as_bytes())
        .map_err(|e| format!(
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

#[cfg(test)]
mod test {
    use super::write_config;
    use rand::random;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::PathBuf;

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
