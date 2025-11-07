use common::error::CommonError;
use std::fmt;

#[derive(Debug)]
pub enum InterpreterError {
    Enrichment,
    Plan,
    Common(CommonError),
    SessionNameRequired,
    SessionTargetRequired,
    WindowTargetRequired,
    PaneTargetRequired,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::Enrichment => write!(f, "Enrichment failed"),
            InterpreterError::Plan => write!(f, "Planning failed"),
            InterpreterError::SessionNameRequired => write!(f, "Session name requried"),
            InterpreterError::SessionTargetRequired => write!(f, "Session target requried"),
            InterpreterError::WindowTargetRequired => write!(f, "Window target requried"),
            InterpreterError::PaneTargetRequired => write!(f, "Pane target requried"),
            InterpreterError::Common(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for InterpreterError {}

impl From<CommonError> for InterpreterError {
    fn from(err: CommonError) -> Self {
        InterpreterError::Common(err)
    }
}
