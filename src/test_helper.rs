use std::rand::random;
use std::io::fs;

/// Test helper to standardize how random files and directories are generated.
pub fn random_name() -> String {
    format!("test_{}", random::<f64>())
}

pub fn cleanup_file(path: &Path) {
    match fs::unlink(path) {
        Ok(()) => (), // succeeded
        Err(e) => println!("Failed to unlink the path {} with error {}", path.display(), e),
    }
}

pub fn cleanup_dir(path: &Path) {
    match fs::rmdir_recursive(path) {
        Ok(()) => (), // succeeded
        Err(e) => println!("Failed to remove the path {} with error {}", path.display(), e),
    }
}
