//! Unix Utility Functions
//! General functions to paper over Unix platform inconsistencies
use crate::error::Error;
use std::process::Command;

pub fn get_stdout(command: &str, args: &[&str]) -> Result<String, Error> {
    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().into())
    } else {
        Err(Error::CommandFailed {
            command: command.to_string(),
            code: output.status.code().unwrap_or(-1),
        })
    }
}

#[inline]
pub fn run(command: &str, args: &[&str]) -> Result<(), Error> {
    get_stdout(command, args).map(|_| ())
}
