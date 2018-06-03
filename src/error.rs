use std::error;
use std::fmt;
use std::result;

use serde::ser;

/// This type represents all possible errors that can occur when serializing
/// CBOR diagnostic notation.
#[derive(Debug, Clone)]
pub enum Error {
    Custom(String),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(msg) => f.write_str(msg)?,
        }
        Ok(())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        ""
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}
