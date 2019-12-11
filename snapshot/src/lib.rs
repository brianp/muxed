//! Muxedsnapshot. A tmux session cloner for Muxed.
extern crate common;
extern crate regex;
extern crate serde;
extern crate serde_yaml;

mod capture;
pub mod tmux;

#[macro_use]
extern crate serde_derive;

use common::args::Args;
use common::first_run::check_first_run;
use common::project_paths::project_paths;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

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
    let session_name = &args.flag_t.as_ref().expect("No TMUX session running");
    let project_paths = project_paths(&args);

    check_first_run(&project_paths.project_directory)?;

    let session = tmux::inspect(&session_name).unwrap();
    let s = serde_yaml::to_string(&session).unwrap();

    write_config(s, &project_paths.project_file, args.flag_f).unwrap();
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

    file.write_all(template.into().as_bytes()).map_err(|e| {
        format!(
            "Could not write contents of template to the file {}. Error {}",
            &path_str, e
        )
    })?;

    file.sync_all()
        .map_err(|e| format!("Could not sync OS data post-write. Error: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod test {
    use common::rand_names;
    use std::fs::File;
    use std::fs;
    use std::io::{Read, Write};
    use super::write_config;

    #[test]
    fn expect_ok_result() {
        let path = rand_names::project_file_with_dir("/tmp");
        let result = write_config("test template", &path, false);
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }

    #[test]
    fn expect_err_result() {
        let path = rand_names::project_file_with_dir("/tmp/non_existent/");
        let result = write_config("test template", &path, false);
        assert!(result.is_err());
    }

    #[test]
    fn expect_file_to_exist() {
        let path = rand_names::project_file_with_dir("/tmp");
        let _ = write_config("test template", &path, false);
        let result = &path.exists();
        let _ = fs::remove_file(&path);
        assert!(result);
    }

    #[test]
    fn expect_file_not_to_exist() {
        let path = rand_names::project_file_with_dir("/tmp/non_existent/");
        let _ = write_config("test template", &path, false);
        assert!(!path.exists());
    }

    #[test]
    fn expect_no_truncation_or_overwrite() {
        // Write a file with content
        let path = rand_names::project_file_with_dir("/tmp");
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
        let path = rand_names::project_file_with_dir("/tmp");
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let result = write_config("new_content", &path, false);

        assert!(result.is_err());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_ok_when_file_exists_using_force() {
        let path = rand_names::project_file_with_dir("/tmp");
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let result = write_config("new_content", &path, true);

        assert!(result.is_ok());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_truncation_or_overwrite_using_force() {
        // Write a file with content
        let path = rand_names::project_file_with_dir("/tmp");
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
    fn expect_file_not_to_exist_using_force_with_bad_dir() {
        let path = rand_names::project_file_with_dir("/tmp/non_existent_path/");
        let _ = write_config("test template", &path, true);
        assert!(!path.exists());
    }

    #[test]
    fn expect_ok_result_using_force_when_file_doesnt_exist() {
        let path = rand_names::project_file_with_dir("/tmp");
        let result = write_config("test template", &path, true);
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }
}
