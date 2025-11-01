//! Muxednew. A Muxed project Template Generator
extern crate common;
extern crate libc;

use common::args::Args;

use common::error::CommonError;
use common::project_paths::ProjectPaths;
use libc::system;
use std::ffi::CString;
use std::fmt::Debug;
use std::{fmt, io};

pub fn edit(args: Args) -> Result<(), EditError> {
    let project_paths = ProjectPaths::try_from(&args)?;
    let command = format!("{} {}", "$EDITOR", project_paths.project_file.display());
    let system_call = CString::new(command).map_err(|_| EditError::SysCall)?;

    unsafe {
        system(system_call.as_ptr());
    };

    Ok(())
}

#[derive(Debug)]
pub enum EditError {
    Common(CommonError),
    Io(io::Error),
    SysCall,
}

impl std::error::Error for EditError {}

impl fmt::Display for EditError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EditError::Io(e) => write!(f, "IO error: {}", e),
            EditError::Common(e) => write!(f, "{}", e),
            EditError::SysCall => write!(f, "Couldn't create the editor open command"),
        }
    }
}

impl From<CommonError> for EditError {
    fn from(err: CommonError) -> EditError {
        EditError::Common(err)
    }
}

impl From<io::Error> for EditError {
    fn from(err: io::Error) -> EditError {
        EditError::Io(err)
    }
}
