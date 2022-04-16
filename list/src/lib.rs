extern crate common;

use common::args::Args;
use common::first_run::check_first_run;
use common::project_paths::{project_paths, CONFIG_EXTENSION};

use std::cmp::Reverse;
use std::path::PathBuf;

pub fn exec(args: Args) -> Result<(), String> {
    let project_paths = project_paths(&args);
    check_first_run(&project_paths.project_directory)?;

    let tmux_session = common::tmux::get_sessions();

    let mut project_names: Vec<String> = project_paths
        .project_directory
        .read_dir()
        .map_err(|_| "Could not read the dir")?
        .filter_map(|path| path.ok())
        .map(|path| PathBuf::from(path.file_name()))
        .filter_map(|buf| match buf.extension().and_then(|x| x.to_str()) {
            Some(CONFIG_EXTENSION) => buf
                .file_stem()
                .and_then(|x| x.to_str())
                .map(|x| x.to_string()),
            _ => None,
        })
        .collect();

    project_names.sort_by_key(|name| (Reverse(tmux_session.get_last_attached(name)), name.clone()));

    let project_displays: Vec<String> = project_names
        .into_iter()
        .map(|name| {
            let mut display = name.clone();

            if tmux_session.has_session(&name) {
                display.push('*');
            }

            if tmux_session.is_attached(&name) {
                display.push_str(" (attached)");
            }

            display
        })
        .collect();

    println!("{}", &project_displays.join("\t\t"));

    Ok(())
}
