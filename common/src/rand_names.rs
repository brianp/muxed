use project_paths::CONFIG_EXTENSION;
use rand::random;
use std::path::PathBuf;

pub fn project_path_name() -> String {
    format!("/tmp/.muxed-test-{}/", random::<u16>())
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

pub fn project_file_with_dir(dir: &str) -> PathBuf {
    let project_file = PathBuf::from(project_file_name()).with_extension(CONFIG_EXTENSION);
    PathBuf::from(dir).join(project_file)
}
