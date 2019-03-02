//! The integration suite for inspecting sessions.

extern crate rand;

mod new {
    use rand::{random};
    use std::process::Command;
    use std::fs::File;
    use std::fs;
    use std::path::PathBuf;
    use std::io::prelude::*;

    pub fn muxednew(project: &String, project_root: &PathBuf) -> () {
        Command::new("./target/debug/muxednew")
            .arg("-p")
            .arg(format!("{}", project_root.display()))
            .arg(project)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    }

    fn setup(project_name: &String) -> (PathBuf, PathBuf) {
        let project_file = format!("/tmp/muxed_{}/{}.yml", random::<u16>(), project_name);
        let project_path = PathBuf::from(&project_file);

        let m = project_path.clone();
        let muxed_path = project_path.parent().unwrap();
        (m, muxed_path.to_path_buf())
    }

    fn cleanup(config_path: &PathBuf) -> () {
        let _ = fs::remove_file(config_path);
        let _ = fs::remove_dir(config_path.parent().unwrap());
    }

    #[test]
    fn creates_new_file_muxed() {
        let project_name = format!("muxed_int_test_{}", random::<u16>());
        let (project_path, muxed_path) = setup(&project_name);
        muxednew(&project_name, &muxed_path);
        assert!(&project_path.exists());
        cleanup(&project_path);
    }
}
