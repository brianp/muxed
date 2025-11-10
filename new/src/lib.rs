//! Muxednew. A Muxed project Template Generator

mod error;

extern crate common;

use crate::error::NewError;
use common::args::Args;
use common::first_run::check_first_run;
use common::project_paths::ProjectPaths;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

static DEFAULT_TEMPLATE: &str = include_str!("template.yml");

/// The main execution method.
/// Accept the name of a project to create a configuration file in the
/// `~/.muxed/` directory.
///
/// # Examples
///
/// You can run the command:
///
/// ```console
/// $ ./muxednew projectName
/// ```
///
/// or specify the directory target of the file:
///
/// ```console
/// $ ./muxednew -p ~/.some_other_dir/ projectName
/// ```
pub fn new(args: Args) -> Result<(), NewError> {
    let project_paths = ProjectPaths::try_from(&args)?;

    check_first_run(&project_paths.project_directory)?;

    let template = if project_paths.template_file.exists() {
        std::fs::read_to_string(project_paths.template_file)?
    } else {
        DEFAULT_TEMPLATE.to_string()
    };

    let replacements = [
        (
            "{file}",
            project_paths
                .project_file
                .to_str()
                .ok_or(NewError::Template(format!(
                    "Couldn't convert the {:?} path into a String to write into the new file",
                    project_paths.project_file
                )))?,
        ),
        ("{project}", &args.arg_project),
    ];

    let new_project = modified_template(&template, &replacements);
    write_template(&new_project, &project_paths.project_file, args.flag_f)?;

    println!(
        "\u{270C} The template file {} has been written to {}\nHappy tmuxing!",
        &project_paths.project_file.display(),
        &project_paths.project_directory.display()
    );
    Ok(())
}

type Replacement<'a, 'b> = (&'a str, &'b str);

fn modified_template(template: &str, replacements: &[Replacement]) -> String {
    let mut template = template.to_string();

    for (placeholder, value) in replacements {
        template = template.replace(placeholder, value);
    }

    template
}

pub fn write_template<S>(template: S, path: &PathBuf, force: bool) -> Result<(), NewError>
where
    S: Into<String>,
{
    let path_str = path
        .to_str()
        .ok_or(NewError::Write("Path could not be opened".to_string()))?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(force)
        .create(force)
        .create_new(!force)
        .open(path)
        .map_err(|e| {
            NewError::Write(format!(
                "Could not create the file {}. Error: {}",
                &path_str, e
            ))
        })?;

    file.write_all(template.into().as_bytes()).map_err(|e| {
        NewError::Write(format!(
            "Could not write contents of template to the file {}. Error {}",
            &path_str, e
        ))
    })?;

    file.sync_all()
        .map_err(|e| NewError::Write(format!("Could not sync OS data post-write. Error: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use common::rand_names;
    use std::fs;
    use std::fs::File;
    use std::path::Path;

    static DEFAULT_TEMPLATE: &str = "file: {file}\nproject: {project}";

    fn file_replacement(path: &Path) -> Replacement<'_, '_> {
        ("{file}", path.to_str().unwrap())
    }

    #[test]
    fn expect_muxed_file_text() {
        let file = PathBuf::from("~/.muxed").join("superProject");
        let value = modified_template(DEFAULT_TEMPLATE, &[file_replacement(&file)]);

        assert!(value.contains("file: ~/.muxed/superProject"));
        assert!(value.contains("project: {project}"));
    }

    #[test]
    fn expect_muxed_project_text() {
        let value = modified_template(DEFAULT_TEMPLATE, &[("{project}", "superProject")]);

        assert!(value.contains("project: superProject"));
    }

    #[test]
    fn expect_no_file_name_placeholder() {
        let file = PathBuf::from("~/.my_dir").join("superProject");
        let value = modified_template(DEFAULT_TEMPLATE, &[file_replacement(&file)]);

        let result = !value.contains("{file}");
        assert!(result);
    }

    #[test]
    fn expect_project_name_with_dir() {
        let file = PathBuf::from("~/.my_dir").join("superProject.yml");
        let value = modified_template(DEFAULT_TEMPLATE, &[file_replacement(&file)]);

        let result = value.contains("file: ~/.my_dir/superProject.yml");
        assert!(result);
    }

    #[test]
    fn expect_project_name_with_dir_and_trailing_slash() {
        let file = PathBuf::from("~/.my_dir/").join("superProject.yml");
        let value = modified_template(DEFAULT_TEMPLATE, &[file_replacement(&file)]);

        let result = value.contains("file: ~/.my_dir/superProject.yml");
        assert!(result);
    }

    #[test]
    fn expect_ok_result_when_path_exists() {
        let path = rand_names::project_file_in_tmp_dir();

        let result = write_template(&"test template".to_string(), &path, false);
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }

    #[test]
    fn expect_err_result_when_path_does_not_exist() {
        let path = rand_names::project_file_path();
        let result = write_template(&"test template".to_string(), &path, false);
        assert!(result.is_err());
    }

    #[test]
    fn expect_new_file_to_exist() {
        let path = rand_names::project_file_in_tmp_dir();
        let _ = write_template(&"test template".to_string(), &path, false);
        let result = &path.exists();
        let _ = fs::remove_file(&path);
        assert!(result);
    }

    #[test]
    fn expect_new_file_not_to_exist() {
        let path = rand_names::project_file_path();
        let _ = write_template(&"test template".to_string(), &path, false);
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
        let _ = write_template(&"new_content".to_string(), &path, false);

        let content = fs::read_to_string(&path).unwrap();

        assert_eq!(content, "original content");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_truncation_or_overwrite() {
        // Write a file with content
        let path = rand_names::project_file_in_tmp_dir();
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let _ = buffer.sync_all();

        // Attempt to create the same named file with new content
        let _ = write_template(&"new content".to_string(), &path, true);

        let content = fs::read_to_string(&path).unwrap();

        assert_eq!(content, "new content");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_err_when_file_exists() {
        let path = rand_names::project_file_in_tmp_dir();
        let mut buffer = File::create(&path).unwrap();
        let _ = buffer.write(b"original content");
        let _ = buffer.sync_all();
        let result = write_template(&"new content".to_string(), &path, false);

        assert!(result.is_err());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn expect_error_on_invalid_path() {
        use std::os::unix::ffi::OsStrExt;
        // Create an invalid UTF-8 path
        let invalid = std::ffi::OsStr::from_bytes(&[0xff, 0xfe, 0xfd]);
        let path = PathBuf::from(invalid);
        let result = write_template("abc", &path, false);
        match result {
            Err(NewError::Write(msg)) => assert!(msg.contains("Path could not be opened")),
            _ => panic!("Expected NewError::Write"),
        }
    }

    #[test]
    fn expect_error_when_path_is_directory() {
        let dir = std::env::temp_dir(); // This directory always exists
        let result = write_template("abc", &dir, false);
        match result {
            Err(NewError::Write(msg)) => assert!(msg.starts_with("Could not create the file")),
            _ => panic!("Expected NewError::Write"),
        }
    }

    #[test]
    fn expect_error_on_permission_denied() {
        let path = PathBuf::from("/root/forbidden_file");
        let result = write_template("abc", &path, false);
        match result {
            Err(NewError::Write(msg)) => assert!(msg.starts_with("Could not create the file")),
            _ => panic!("Expected NewError::Write"),
        }
    }
}
