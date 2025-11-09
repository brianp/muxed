//! Muxedsnapshot. A tmux session cloner for Muxed.
extern crate common;
extern crate new;
extern crate regex;
extern crate serde;
extern crate serde_yaml;

mod capture;
mod error;
pub mod tmux;

use crate::error::SnapshotError;
use common::args::Args;
use common::first_run::check_first_run;
use common::project_paths::ProjectPaths;
use new::write_template as write_config;

/// The main execution method.
/// Accepts two arguments. -n for the name of the project file and -t to target
/// the session.
///
/// # Examples
///
/// You can run the command:
///
/// ```console
/// $ ./muxedsnapshot -n my_new_project -t existing_sesion
/// ```
///
/// or
///
/// ```console
/// $ ./muxed snapshot -n jasper -t 1
/// ```
pub fn snapshot(args: Args) -> Result<(), SnapshotError> {
    let session_name = args
        .flag_t
        .as_ref()
        .ok_or(SnapshotError::NoSessionRunning)?;
    let project_paths = ProjectPaths::try_from(&args)?;

    check_first_run(&project_paths.project_directory)?;

    let session = tmux::inspect(session_name).unwrap();
    let s = serde_yaml::to_string(&session).unwrap();

    write_config(s, &project_paths.project_file, args.flag_f).unwrap();
    println!("We made a snapshot of your session! \u{1F60A}");
    Ok(())
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;
    use super::write_config;
    use common::rand_names;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn expect_ok_result() {
        let path = rand_names::project_file_in_tmp_dir();
        let result = write_config("test template", &path, false);
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }

    #[test]
    fn expect_err_result() {
        let path = rand_names::project_file_with_dir("/non_existent/");
        let result = write_config("test template", &path, false);
        assert!(result.is_err());
    }

    #[test]
    fn expect_file_to_exist() {
        let path = rand_names::project_file_in_tmp_dir();
        let _ = write_config("test template", &path, false);
        let result = &path.exists();
        let _ = fs::remove_file(&path);
        assert!(result);
    }

    #[test]
    fn expect_file_not_to_exist() {
        let path = rand_names::project_file_with_dir("/non_existent/");
        let _ = write_config("test template", &path, false);
        assert!(!path.exists());
    }

    #[test]
    fn expect_no_truncation_or_overwrite() {
        // Write a file with content
        let path = rand_names::project_file_in_tmp_dir();
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let _ = buffer.sync_all();

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
        let path = rand_names::project_file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        println!("{:?}", path);
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let _ = buffer.sync_all();
        let result = write_config("new_content", &path, false);

        assert!(result.is_err());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_ok_when_file_exists_using_force() {
        let path = rand_names::project_file_in_tmp_dir();
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let _ = buffer.sync_all();
        let result = write_config("new_content", &path, true);

        assert!(result.is_ok());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_truncation_or_overwrite_using_force() {
        // Write a file with content
        let path = rand_names::project_file_in_tmp_dir();
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let _ = buffer.sync_all();

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
        let path = rand_names::project_file_with_dir("/non_existent_path/");
        let _ = write_config("test template", &path, true);
        assert!(!path.exists());
    }

    #[test]
    fn expect_ok_result_using_force_when_file_doesnt_exist() {
        let path = rand_names::project_file_in_tmp_dir();
        let result = write_config("test template", &path, true);
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }
}
