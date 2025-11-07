use std::fmt;

#[derive(Debug)]
pub enum CommonError {
    ProjectPaths(String),
    FirstRun(String),
    Target,
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommonError::ProjectPaths(msg) => write!(f, "ProjectPaths error: {}", msg),
            CommonError::FirstRun(msg) => write!(f, "FirstRun error: {}", msg),
            CommonError::Target => write!(f, "Tried to build an incompatible target"),
        }
    }
}

impl std::error::Error for CommonError {}
