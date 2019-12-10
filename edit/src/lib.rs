//! Muxednew. A Muxed project Template Generator
extern crate common;
extern crate libc;

use common::args::Args;
use common::project_paths::project_paths;

use libc::system;
use std::ffi::CString;
use std::io;

pub fn exec(args: Args) -> Result<(), io::Error> {
    let project_paths = project_paths(&args);
    let command = format!("{} {}", "$EDITOR", project_paths.project_file.display());
    let system_call = CString::new(command).expect("Couldn't create the editor open command");

    unsafe {
        system(system_call.as_ptr());
    };

    Ok(())
}
