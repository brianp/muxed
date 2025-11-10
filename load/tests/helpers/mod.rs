//! The integration suite helpers.

use common::args::Args;
use common::rand_names;
use common::tmux::Session;
use rand::random;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn project_name(contents: &[u8]) -> String {
    let string_content = str::from_utf8(contents).unwrap();
    let session: Session = serde_saphyr::from_str(string_content).unwrap();

    match session.name {
        Some(x) => x.to_string(),
        None => rand_names::project_file_name(),
    }
}

fn setup(contents: &[u8]) -> (String, PathBuf) {
    let project_name = project_name(contents);
    let project_path = rand_names::project_file_path_with_name(&project_name);

    let muxed_path = project_path.parent().unwrap();
    if !muxed_path.exists() {
        fs::create_dir(muxed_path).unwrap();
    };

    let mut buffer = File::create(&project_path).unwrap();
    let _ = buffer.write(contents);
    let _ = buffer.sync_all();

    (project_name, project_path)
}

fn cleanup(project_name: &str, config_path: &PathBuf) {
    let _ = fs::remove_file(config_path);
    let _ = fs::remove_dir(config_path.parent().unwrap());
    kill_session(project_name);
}

pub fn test_with_contents(contents: &[u8]) -> Session {
    let (project_name, config_path) = setup(contents);
    let _ = open_muxed(&project_name, config_path.parent().unwrap());

    let completed = PathBuf::from(format!(
        "/tmp/{}-{}.complete",
        project_name,
        random::<u16>()
    ));
    let exec = format!("touch '{}'", completed.display());

    send_keys(&project_name, &exec);
    wait_on(&completed);

    let session = snapshot::inspect(&project_name).unwrap();
    cleanup(&project_name, &config_path);
    session
}

fn open_muxed(project: &str, project_root: &Path) -> Result<(), String> {
    let args = Args {
        arg_project: project.to_string(),
        flag_p: Some(format!("{}", project_root.display())),
        flag_debug: true,
        ..Default::default()
    };

    load::load(args).map_err(|e| format!("Open error {}", e))
}

fn kill_session(target: &str) {
    let _ = load::tmux::call(&["kill-session", "-t", target]);
}

fn send_keys(target: &str, exec: &str) {
    let _ = load::tmux::call(&["send-keys", "-t", target, exec, "KPEnter"]);
}

fn wait_on(file: &PathBuf) {
    while !file.exists() {
        // Wait increased from 10 to 750 due to the pre_window tests.
        sleep(Duration::from_millis(750));
    }
}
