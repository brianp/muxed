use crate::project_paths::CONFIG_EXTENSION;
use rand::random;
use std::env::temp_dir;
use std::path::PathBuf;

pub fn project_path_name() -> String {
    let tmp = std::env::temp_dir().join(format!(".muxed-test-{}", random::<u16>()));
    tmp.to_str().unwrap().to_string()
}

pub fn template_path_name() -> String {
    format!("muxed-test-template-{}", random::<u16>())
}

pub fn project_path() -> PathBuf {
    PathBuf::from(project_path_name())
}

pub fn project_file_name() -> String {
    format!("muxed-test-project-{}", random::<u16>())
}

pub fn project_file_path() -> PathBuf {
    let project_file = PathBuf::from(project_file_name()).with_extension(CONFIG_EXTENSION);
    project_path().join(project_file)
}

pub fn project_file_path_with_name(name: &str) -> PathBuf {
    let project_file = PathBuf::from(name).with_extension(CONFIG_EXTENSION);
    project_path().join(project_file)
}

pub fn project_template_file_path() -> PathBuf {
    let project_file = PathBuf::from(template_path_name()).with_extension(CONFIG_EXTENSION);
    project_path().join(project_file)
}

pub fn project_file_with_dir(dir: &str) -> PathBuf {
    let project_file = PathBuf::from(project_file_name()).with_extension(CONFIG_EXTENSION);
    PathBuf::from(dir).join(project_file)
}

pub fn project_file_in_tmp_dir() -> PathBuf {
    let project_file = PathBuf::from(project_file_name()).with_extension(CONFIG_EXTENSION);
    temp_dir().join(project_file)
}
