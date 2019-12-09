//! The integration suite for inspecting sessions.

extern crate common;
extern crate new;
extern crate rand;

#[cfg(test)]
mod test {
    mod new {
        use common::args::Args;
        use new;
        use rand::random;
        use std::fs;
        use std::path::PathBuf;

        pub fn new(project: &str, project_root: &PathBuf) -> Result<(), String> {
            let args = Args {
                flag_debug: false,
                flag_dryrun: false,
                flag_d: true,
                flag_v: false,
                flag_f: false,
                flag_p: Some(format!("{}", project_root.display())),
                flag_t: None,
                arg_project: project.to_string(),
                cmd_new: false,
                cmd_snapshot: false,
            };

            new::exec(args)
        }

        fn setup(project_name: &str) -> (PathBuf, PathBuf) {
            let project_file = format!("/tmp/muxed_{}/{}.yml", random::<u16>(), project_name);
            let project_path = PathBuf::from(&project_file);

            let m = project_path.clone();
            let muxed_path = project_path.parent().unwrap();
            (m, muxed_path.to_path_buf())
        }

        fn cleanup(config_path: &PathBuf) {
            let _ = fs::remove_file(config_path);
            let _ = fs::remove_dir(config_path.parent().unwrap());
        }

        #[test]
        fn creates_new_file_muxed() {
            let project_name = format!("muxed_int_test_{}", random::<u16>());
            let (project_path, muxed_path) = setup(&project_name);
            let _ = new(&project_name, &muxed_path);
            assert!(&project_path.exists());
            cleanup(&project_path);
        }
    }
}
