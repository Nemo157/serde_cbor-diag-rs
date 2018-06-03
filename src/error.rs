use std::{error, fmt, io, result};

use serde::ser;

/// This type represents all possible errors that can occur when serializing
/// CBOR diagnostic notation.
#[derive(Debug)]
pub enum Error {
    Custom(String),
    Io(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(msg) => f.write_str(msg)?,
            Error::Io(err) => err.fmt(f)?,
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

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
