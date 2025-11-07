use crate::command::Commands;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::to_command::Plan;
use crate::project::Project;

type Result<T> = std::result::Result<T, InterpreterError>;

pub fn plan(project: &Project) -> Result<Vec<Commands>> {
    project.session().command_plan()
}
