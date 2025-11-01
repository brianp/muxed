use common::error::CommonError;
use std::ffi::NulError;
use std::{fmt, io};

#[derive(Debug)]
pub enum TmuxError {
    Io(io::Error),
    Common(CommonError),
    Attach(NulError),
    Pre,
    Config,
}

impl fmt::Display for TmuxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TmuxError::Io(e) => write!(f, "IO error: {}", e),
            TmuxError::Common(e) => write!(f, "{}", e),
            TmuxError::Attach(e) => write!(f, "Couldn't attach to TMUX: {}", e),
            TmuxError::Pre => write!(f, "Couldn't find args for pre option"),
            TmuxError::Config => write!(f, "Couldn't get tmux options"),
        }
    }
}

impl std::error::Error for TmuxError {}

impl From<CommonError> for TmuxError {
    fn from(err: CommonError) -> TmuxError {
        TmuxError::Common(err)
    }
}

impl From<io::Error> for TmuxError {
    fn from(err: io::Error) -> TmuxError {
        TmuxError::Io(err)
    }
}

impl From<NulError> for TmuxError {
    fn from(err: NulError) -> TmuxError {
        TmuxError::Attach(err)
    }
}
