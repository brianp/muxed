use common::error::CommonError;
use std::{fmt, io};

#[derive(Debug)]
pub enum SnapshotError {
    Common(CommonError),
    Io(io::Error),
    NoSessionRunning,
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnapshotError::Common(e) => write!(f, "{}", e),
            SnapshotError::Io(e) => write!(f, "{}", e),
            SnapshotError::NoSessionRunning => write!(f, "No TMUX session was found running"),
        }
    }
}

impl std::error::Error for SnapshotError {}

impl From<CommonError> for SnapshotError {
    fn from(err: CommonError) -> SnapshotError {
        SnapshotError::Common(err)
    }
}

impl From<io::Error> for SnapshotError {
    fn from(err: io::Error) -> SnapshotError {
        SnapshotError::Io(err)
    }
}
