use common::error::CommonError;
use new::error::NewError;
use std::{fmt, io};

#[derive(Debug)]
pub enum SnapshotError {
    Common(CommonError),
    Io(io::Error),
    New(NewError),
    SerdeJson(serde_json::Error),
    SessionTargetRequired,
    ToPaneFailed,
    ToWindowFailed,
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnapshotError::Common(e) => write!(f, "{}", e),
            SnapshotError::Io(e) => write!(f, "{}", e),
            SnapshotError::New(e) => write!(f, "{}", e),
            SnapshotError::SerdeJson(e) => write!(f, "{}", e),
            SnapshotError::SessionTargetRequired => write!(f, "No TMUX session was provided"),
            SnapshotError::ToPaneFailed => write!(f, "Failed to create pane from snapshot"),
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

impl From<NewError> for SnapshotError {
    fn from(err: NewError) -> SnapshotError {
        SnapshotError::New(err)
    }
}
