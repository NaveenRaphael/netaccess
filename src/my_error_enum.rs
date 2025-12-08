use std::{error::Error, fmt::Display};

use fantoccini::error::CmdError;

#[derive(Debug)]
pub enum MyError {
    Fanto(fantoccini::error::CmdError),
    Cred(String),
}
impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Fanto(cmd_error) => write!(f, "{cmd_error}"),
            MyError::Cred(s) => write!(f, "{s}"),
        }
    }
}
impl Error for MyError {}
impl From<CmdError> for MyError {
    fn from(value: CmdError) -> Self {
        MyError::Fanto(value)
    }
}
