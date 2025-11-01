mod error;

extern crate common;

use common::args::Args;
use common::first_run::check_first_run;
use common::project_paths::{CONFIG_EXTENSION, ProjectPaths};

use crate::error::ListError;
use std::path::PathBuf;

pub fn list(args: Args) -> Result<(), ListError> {
    let project_paths = ProjectPaths::try_from(&args)?;
    check_first_run(&project_paths.project_directory)?;

    let mut projects: Vec<String> = project_paths
        .project_directory
        .read_dir()?
        .filter_map(|path| path.ok())
        .filter(|path| path.path() != project_paths.template_file)
        .map(|path| PathBuf::from(path.file_name()))
        .filter_map(|buf| match buf.extension().and_then(|x| x.to_str()) {
            Some(CONFIG_EXTENSION) => buf
                .file_stem()
                .and_then(|x| x.to_str())
                .map(|x| x.to_string()),
            _ => None,
        })
        .collect();

    projects.sort();

    let delimiter = if !atty::is(atty::Stream::Stdout) || args.flag_1 {
        "\n"
    } else {
        "\t\t"
    };

    println!("{}", &projects.join(delimiter));

    Ok(())
}
