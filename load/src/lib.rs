extern crate dirs;
extern crate libc;
extern crate yaml_rust;

extern crate common;

pub mod command;
pub mod error;
mod parser;
pub mod project;
pub mod tmux;

use crate::error::LoadError;
use args::Args;
use command::Commands;
use common::project_paths::ProjectPaths;
use common::{args, first_run};
use tmux::config::Config;

type Result<T> = std::result::Result<T, LoadError>;

pub fn load(args: Args) -> Result<()> {
    let project_paths = ProjectPaths::try_from(&args)?;

    let project = project::read(&args.arg_project, project_paths)?;
    let project_name = project.name();

    let commands: Vec<Commands> = match project::session_exists(&project_name) {
        Some(c) => {
            vec![c]
        }
        None => {
            let config = Config::from_string(tmux::get_config()?);
            parser::call(project.yaml(), &project_name, args.flag_d, &config)
                .map_err(LoadError::Parse)?
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
