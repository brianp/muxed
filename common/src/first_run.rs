use std::fs::create_dir;
use std::path::Path;

/// Used just to check for the existence of the default path. Prints out
/// useful messages as to what's happening.
///
/// # Examples
///
/// ```rust
/// extern crate rand;
///
/// use common::first_run::check_first_run;
/// use rand::random;
/// use std::fs::{create_dir, remove_dir};
/// use std::path::{Path, PathBuf};
///
/// let path_name = format!("/tmp/.muxed-{}/", random::<u16>());
/// let path = Path::new(&path_name);
/// assert!(!path.exists());
///
/// check_first_run(path);
///
/// assert!(path.exists());
///
/// let _ = remove_dir(path);
/// ```
pub fn check_first_run(muxed_dir: &Path) -> Result<(), String> {
    if !muxed_dir.exists() {
        create_dir(muxed_dir).map_err(|e| format!("We noticed the configuration directory: `{}` didn't exist so we tried to create it, but something went wrong: {}", muxed_dir.display(), e))?;
        println!(
            "Looks like this is your first time here. Muxed could't find the configuration directory: `{}`",
            muxed_dir.display()
        );
        println!("Creating that now \u{1F44C}\n")
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rand_names;
    use std::fs::remove_dir;

    #[test]
    fn creates_dir_if_not_exist() {
        let path = rand_names::project_path();

        assert!(!path.exists());
        assert!(check_first_run(&path).is_ok()); // Side effects
        assert!(path.exists());

        let _ = remove_dir(path);
    }

    #[test]
    fn returns_ok_if_already_exists() {
        let path = rand_names::project_path();
        let _ = create_dir(&path);

        assert!(check_first_run(&path).is_ok());

        let _ = remove_dir(path);
    }
}
