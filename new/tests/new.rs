//! The integration suite for inspecting sessions.

extern crate common;
extern crate new;

#[cfg(test)]
mod test {
    mod new {
        use common::args::Args;
        use common::rand_names;
        use new;
        use std::fs;
        use std::path::Path;

        pub fn new(project_path: &Path) -> Result<(), String> {
            let args = Args {
                flag_p: Some(project_path.parent().unwrap().display().to_string()),
                arg_project: project_path.file_name().unwrap().to_str().unwrap().to_string(),
                cmd_new: true,
                ..Default::default()
            };

            new::exec(args)
        }

        fn cleanup(config_path: &Path) {
            let _ = fs::remove_file(config_path);
            let _ = fs::remove_dir(config_path.parent().unwrap());
        }

        #[test]
        fn creates_new_file() {
            let project_path = rand_names::project_file_path();

            assert!(new(&project_path).is_ok());
            assert!(&project_path.exists());

            cleanup(&project_path);
        }
    }
}
