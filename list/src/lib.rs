extern crate common;

use common::args::Args;
use common::first_run::check_first_run;
use common::project_paths::project_paths;

use std::path::PathBuf;

pub fn exec(args: Args) -> Result<(), String> {
    let project_paths = project_paths(&args);
    check_first_run(&project_paths.project_directory)?;

    let projects: Vec<String> = project_paths
        .project_directory
        .read_dir()
        .map_err(|_| "Could not read the dir")?
        .filter_map(|path| path.ok())
        .map(|path| PathBuf::from(path.file_name()))
        .filter_map(|buf| match buf.extension().and_then(|x| x.to_str()) {
            Some("yml") => buf
                .file_stem()
                .and_then(|x| x.to_str())
                .map(|x| x.to_string()),
            _ => None,
        })
        .collect();

    println!("{}", projects.join(" "));

    Ok(())
}
