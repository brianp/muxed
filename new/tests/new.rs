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

        fn cleanup(config_path: &Path) {
            let _ = fs::remove_file(config_path);
            let _ = fs::remove_dir(config_path.parent().unwrap());
        }

        #[test]
        fn creates_new_file() {
            let project_path = rand_names::project_file_path();

            let args = Args {
                arg_project: project_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                cmd_new: true,
                flag_p: Some(project_path.parent().unwrap().display().to_string()),
                ..Default::default()
            };

            assert!(new::new(args).is_ok());
            assert!(&project_path.exists());

            cleanup(&project_path);
        }

        #[test]
        fn creates_new_file_from_template() {
            let project_path = rand_names::project_file_path();

            let template_path = rand_names::project_template_file_path();
            let _ = fs::create_dir(template_path.parent().as_ref().unwrap());
            fs::write(
                &template_path,
                "This is a fake template file. {file} {project}",
            )
            .unwrap();

            let project = project_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let args = Args {
                arg_project: project.clone(),
                cmd_new: true,
                flag_p: Some(project_path.parent().unwrap().display().to_string()),
                flag_template: Some(template_path.display().to_string()),
                ..Default::default()
            };

            assert!(new::new(args).is_ok());
            assert!(&project_path.exists());

            let new_contents = fs::read_to_string(&project_path).unwrap();
            assert_eq!(
                new_contents,
                format!(
                    "This is a fake template file. {} {}",
                    project_path.display(),
                    project
                )
            );

            cleanup(&project_path);
        }

        #[test]
        fn overwrites_existing_file() {
            let project_path = rand_names::project_file_path();
            let _ = fs::create_dir(project_path.parent().as_ref().unwrap());
            let _ = fs::File::create(&project_path);

            let args = Args {
                arg_project: project_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                cmd_new: true,
                flag_f: true,
                flag_p: Some(project_path.parent().unwrap().display().to_string()),
                ..Default::default()
            };

            assert!(new::new(args).is_ok());

            let contents = fs::read_to_string(&project_path).unwrap();

            let name = format!("# {}", project_path.display());
            assert!(contents.contains(&name));

            cleanup(&project_path);
        }

        #[test]
        fn fails_to_write_over_existing_file() {
            let project_path = rand_names::project_file_path();
            let _ = fs::create_dir(project_path.parent().as_ref().unwrap());
            let _ = fs::File::create(&project_path);

            let args = Args {
                arg_project: project_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                cmd_new: true,
                flag_p: Some(project_path.parent().unwrap().display().to_string()),
                ..Default::default()
            };

            assert!(new::new(args).is_err());

            cleanup(&project_path);
        }
    }
}
