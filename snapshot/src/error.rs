use common::error::CommonError;
use std::{fmt, io};

#[derive(Debug)]
pub enum SnapshotError {
    Common(CommonError),
    Io(io::Error),
    SessionTargetRequired,
    SerdeJson(serde_json::Error),
    ToWindowFailed,
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnapshotError::Common(e) => write!(f, "{}", e),
            SnapshotError::Io(e) => write!(f, "{}", e),
            SnapshotError::SessionTargetRequired => write!(f, "No TMUX session was provided"),
            SnapshotError::SerdeJson(e) => write!(f, "{}", e),
            SnapshotError::ToWindowFailed => write!(f, "Failed to create window from snapshot"),
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

impl From<serde_json::Error> for SnapshotError {
    fn from(err: serde_json::Error) -> SnapshotError {
        SnapshotError::SerdeJson(err)
    }
}
