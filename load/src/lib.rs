extern crate dirs;
extern crate libc;
extern crate yaml_rust;

extern crate common;

pub mod command;
pub mod error;
mod interpreter;
pub mod project;
pub mod tmux;

use crate::error::LoadError;
use args::Args;
use command::Commands;

use common::project_paths::ProjectPaths;
use common::tmux::Config;
use common::{args, first_run};

type Result<T> = std::result::Result<T, LoadError>;

pub fn load(args: Args) -> Result<()> {
    let project_paths = ProjectPaths::try_from(&args)?;

    let mut project = project::read(&args.arg_project, project_paths)?;
    let name = project.name().to_string();

    let commands: Vec<Commands> = match project::session_exists(project.name()) {
        Some(c) => {
            vec![c]
        }
        None => {
            let config = Config::from_string(tmux::get_config()?);
            interpreter::enrich(project.session_mut(), name, args.flag_d, config);
            interpreter::plan(&project)?
        }
    };

    if args.flag_debug {
        println!("{:?}", &commands);
    };

    for command in &commands {
        command.as_trait().call(args.flag_debug)?;
    }

    Ok(())
}
