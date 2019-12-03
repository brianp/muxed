//! The integration suite helpers.

use common::args::Args;
use rand::random;
use snapshot::tmux;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str;
use std::thread::sleep;
use std::time::Duration;
use yaml_rust::YamlLoader;

fn project_name(contents: &[u8]) -> String {
    let string_content = str::from_utf8(contents).unwrap();
    let yaml = YamlLoader::load_from_str(string_content).unwrap();

    match yaml[0]["name"].as_str() {
        Some(x) => x.to_string(),
        None => format!("muxed_int_test_{}", random::<u16>()),
    }
}

fn setup(contents: &[u8]) -> (String, PathBuf) {
    let project_name = project_name(contents);
    let project_file = format!("/tmp/muxed_{}/{}.yml", random::<u16>(), project_name);
    let project_path = PathBuf::from(&project_file);

    let muxed_path = project_path.parent().unwrap();
    if !muxed_path.exists() {
        println!("{:?}", fs::create_dir(muxed_path))
    };

    let mut buffer = File::create(&project_path).unwrap();
    let _ = buffer.write(contents);

    (project_name, project_path.clone())
}

fn cleanup(project_name: &str, config_path: &PathBuf) {
    let _ = fs::remove_file(config_path);
    let _ = fs::remove_dir(config_path.parent().unwrap());
    kill_session(project_name);
}

pub fn test_with_contents(contents: &[u8]) -> snapshot::tmux::session::Session {
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

    let session = tmux::inspect(&project_name).unwrap();
    cleanup(&project_name, &config_path);
    session
}

fn open_muxed(project: &str, project_root: &Path) -> Result<(), String> {
    let args = Args {
        flag_d: true,
        flag_v: false,
        flag_f: false,
        flag_p: Some(format!("{}", project_root.display())),
        flag_t: None,
        arg_project: project.to_string(),
        cmd_new: false,
        cmd_snapshot: false,
    };

    load::exec(args)
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
