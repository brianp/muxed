//! The struct managing cli args
use rand::random;
use serde::Deserialize;

/// The args struct for taking arguments passed in from the command line
/// and making it easier to pass around.
/// `flag_d` is whether the session should be daemonzied
/// `flag_v` to print the version
/// `flag_f` to force overwrite of a file
/// `flag_p` the project directory to read or write to
/// `flag_t` the session to read from
/// `flag_debug` run inline print statements for debugging
/// `arg_project` the project file to read
/// `cmd_edit` if `true` run edit command
/// `cmd_new` if `true` run new command
/// `cmd_snapshot` if `true` run snapshot command
///
#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_debug: bool,
    pub flag_d: bool,
    pub flag_f: bool,
    pub flag_p: Option<String>,
    pub flag_t: Option<String>,
    pub flag_v: bool,
    pub arg_project: String,
    pub cmd_edit: bool,
    pub cmd_new: bool,
    pub cmd_snapshot: bool,
}

impl Default for Args {
    fn default() -> Self {
        let name = format!("{}", random::<u16>());

        Args {
            arg_project: name,
            cmd_edit: false,
            cmd_new: true,
            cmd_snapshot: false,
            flag_d: true,
            flag_debug: false,
            flag_f: false,
            flag_p: None,
            flag_t: None,
            flag_v: false,
        }
    }
}
