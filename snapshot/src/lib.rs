//! Muxedsnapshot. A tmux session cloner for Muxed.
extern crate common;
extern crate new;
extern crate regex;
extern crate serde;
extern crate sysinfo;

mod entity;
mod error;
pub mod session_data;

use crate::error::SnapshotError;
use crate::session_data::SessionOutput;
use common::DEBUG;
use common::args::Args;
use common::first_run::check_first_run;
use common::project_paths::ProjectPaths;
use common::tmux::{Session, Target};
use new::write_template as write_config;
use std::process::{Command, Stdio};
use std::result;

static WINDOW_FORMAT: &str = r##"{"type": "window", "session":"#S", "index":#I,"name":"#W","active":#{window_active},"layout":"#{window_layout}"}"##;
static PANE_FORMAT: &str = r##"{"type": "pane", "session":"#S", "window_index":#I,"index":#P,"active":#{pane_active},"path":"#{pane_current_path}", "pid":#{pane_pid}}"##;

type Result<T> = result::Result<T, SnapshotError>;

/// The main execution method.
/// Accepts two arguments. -n for the name of the project file and -t to target
/// the session.
///
/// # Examples
///
/// You can run the command:
///
/// ```console
/// $ ./muxedsnapshot -n my_new_project -t existing_session
/// ```
///
/// or
///
/// ```console
/// $ ./muxed snapshot -n jasper -t 1
/// ```
pub fn snapshot(args: Args) -> Result<()> {
    let session_name = args
        .flag_t
        .as_ref()
        .ok_or(SnapshotError::SessionTargetRequired)?;
    let project_paths = ProjectPaths::try_from(&args)?;

    check_first_run(&project_paths.project_directory)?;

    let session = inspect(session_name)?;
    let s = serde_saphyr::to_string(&session).unwrap();

    write_config(s, &project_paths.project_file, args.flag_f).unwrap();
    println!("We made a snapshot of your session! \u{1F60A}");

    Ok(())
}

pub fn inspect(name: &str) -> result::Result<Session, SnapshotError> {
    let target = Target::new(name, None, None);
    let session_data = session_data(&target)?;

    if DEBUG.load() {
        dbg!(&session_data);
    }

    Session::try_from(session_data)
}

fn session_data(target: &Target) -> Result<SessionOutput> {
    let output = Command::new("tmux")
        .args([
            "list-windows",
            "-t",
            target.combined.as_str(),
            "-F",
            WINDOW_FORMAT,
            ";",
        ])
        .args([
            "list-panes",
            "-s",
            "-t",
            target.combined.as_str(),
            "-F",
            PANE_FORMAT,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(SessionOutput {
        output,
        target: target.clone(),
    })
}

#[cfg(test)]
mod test {
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
