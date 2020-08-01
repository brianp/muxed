//! The integration suite for inspecting sessions.

extern crate common;
extern crate list;

#[cfg(test)]
mod test {
    mod list {
        use common::args::Args;
        use common::rand_names;
        use list;
        use std::fs;
        use std::fs::File;
        use std::path::PathBuf;

        fn make_files() -> PathBuf {
            let dir_name = rand_names::project_path_name();
            let dir = PathBuf::from(&dir_name);
            
            dbg!(&dir);
            if !&dir.exists() {
                println!("{:?}", fs::create_dir(&dir))
            };

            let configs = ["foo.yml", "bar.yml", "muxed.yml"];
            for config in &configs {
              let path = PathBuf::from(&dir_name).join(config);
              let _ = File::create(&path).expect("Muxed list test failed to create test config files.");
            }

            dir
        }

        fn cleanup(config_path: PathBuf) {
            // TODO: Do I really want to do this? What if we get it wrong.
            let _ = fs::remove_dir_all(config_path);
        }

        #[test]
        fn lists_files_muxed_dir() {
            // Uhhh I haven't captured any stdout data here. So it really just
            // tests to ensure it runs. Either capture stdout, or break the
            // functions down to test the collection of files, and the filtering.
            let project_dir = make_files();

            let args = Args {
                cmd_list: true,
                flag_p: Some(project_dir.display().to_string()),
                ..Default::default()
            };

            assert!(list::exec(args).is_ok());

            cleanup(project_dir);
        }
    }
}
