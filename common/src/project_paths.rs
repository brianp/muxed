use args::Args;
#[cfg(not(any(test, doctest)))]
use dirs::home_dir;
use std::path::PathBuf;

pub const CONFIG_EXTENSION: &str = "yml";
static MUXED_FOLDER: &str = ".muxed";

pub struct ProjectPaths {
    pub home_directory: PathBuf,
    pub project_directory: PathBuf,
    pub project_file: PathBuf,
}

impl ProjectPaths {
    pub fn new(
        home_directory: PathBuf,
        project_directory: PathBuf,
        project_file: PathBuf,
    ) -> ProjectPaths {
        ProjectPaths {
            home_directory,
            project_directory,
            project_file,
        }
    }

    pub fn from_strs(
        home_directory: &str,
        project_directory: &str,
        project_file: &str,
    ) -> ProjectPaths {
        let home_directory = PathBuf::from(home_directory);
        let project_directory = home_directory.join(project_directory);
        let project_file = project_directory
            .join(project_file)
            .with_extension(CONFIG_EXTENSION);

        ProjectPaths {
            home_directory,
            project_directory,
            project_file,
        }
    }
}

/// A common method for returning the project directory and filepath. The method
/// will check for a passed argument set with -p but if it does not exist will
/// map the path for the .muxed directory in the users home directory and return
/// that as the default.
///
/// # Examples
///
/// #cfg(doctest) isn't working. This results in different home dirs
/// ```rust,no_run
/// use common::project_paths::{ProjectPaths, project_paths};
/// use common::args::Args;
/// use std::path::PathBuf;
///
/// let args = Args {
///     arg_project: "projectname".to_string(),
///     ..Default::default()
/// };
///
/// let project_paths = project_paths(&args);
///
/// let paths = ProjectPaths::new(
///     PathBuf::from("/tmp"),
///     PathBuf::from("/tmp/.muxed"),
///     PathBuf::from("/tmp/.muxed/projectname.yml")
/// );
///
/// assert_eq!(project_paths.home_directory, PathBuf::from("/tmp"));
/// assert_eq!(project_paths.project_directory, PathBuf::from("/tmp/.muxed"));
/// assert_eq!(project_paths.project_file, PathBuf::from("/tmp/.muxed/projectname.yml"))
/// ```
pub fn project_paths(args: &Args) -> ProjectPaths {
    let homedir = homedir().expect("We couldn't find your home directory.");
    let default_dir = homedir.join(MUXED_FOLDER);
    let project_directory = args
        .flag_p
        .as_ref()
        .map_or(default_dir, |p| PathBuf::from(p));

    let project_filename = PathBuf::from(&args.arg_project).with_extension(CONFIG_EXTENSION);
    let project_fullpath = project_directory.join(project_filename);

    ProjectPaths::new(homedir, project_directory, project_fullpath)
}

/// A Thin wrapper around the home_dir crate. This is so we can swap the default
/// dir out during testing.
#[cfg(not(any(test, doctest)))]
fn homedir() -> Option<PathBuf> {
    home_dir()
}

/// Return the temp dir as the users home dir during testing.
#[cfg(any(test, doctest))]
fn homedir() -> Option<PathBuf> {
    Some(PathBuf::from("/tmp"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expects_tmp_as_default_homedir() {
        let args: Args = Default::default();
        let project_paths = project_paths(&args);

        assert_eq!(project_paths.home_directory, PathBuf::from("/tmp"))
    }

    #[test]
    fn expects_muxed_as_default_project_dir() {
        let args: Args = Default::default();
        let project_paths = project_paths(&args);

        assert_eq!(
            project_paths.project_directory,
            PathBuf::from("/tmp/.muxed")
        )
    }

    #[test]
    fn expects_spacey_as_project_dir() {
        let args = Args {
            flag_p: Some("/spacey".to_string()),
            ..Default::default()
        };
        let project_paths = project_paths(&args);

        assert_eq!(project_paths.project_directory, PathBuf::from("/spacey"))
    }

    #[test]
    fn expects_projectname_as_yml_file() {
        let args = Args {
            arg_project: "projectname".to_string(),
            ..Default::default()
        };
        let project_paths = project_paths(&args);

        assert_eq!(
            project_paths.project_file,
            PathBuf::from("/tmp/.muxed/projectname.yml")
        )
    }
}
