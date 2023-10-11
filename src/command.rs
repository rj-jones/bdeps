use std::fmt;
use std::io;
use std::process::Command;

#[derive(Debug)]
pub enum CommandError {
    SpawnError(io::Error),
    WaitError(io::Error),
    NonZeroExit(i32),
    Terminated,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::SpawnError(e) => write!(f, "Spawn error: {}", e),
            CommandError::WaitError(e) => write!(f, "Wait error: {}", e),
            CommandError::NonZeroExit(code) => write!(f, "Non-zero exit code: {}", code),
            CommandError::Terminated => write!(f, "Process terminated"),
        }
    }
}

impl From<io::Error> for CommandError {
    fn from(error: io::Error) -> Self {
        CommandError::SpawnError(error)
    }
}

pub fn execute_command(command: &mut Command) -> Result<(), CommandError> {
    let mut child = command.spawn().map_err(CommandError::SpawnError)?;

    match child.wait() {
        Ok(exit_status) => {
            if !exit_status.success() {
                match exit_status.code() {
                    Some(code) => Err(CommandError::NonZeroExit(code)),
                    None => Err(CommandError::Terminated),
                }
            } else {
                Ok(())
            }
        }
        Err(e) => Err(CommandError::WaitError(e)),
    }
}
