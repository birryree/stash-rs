//! This unifies multiple classes of errors

use std::error::Error as StdError;
use std::io::Error as IoError;
use serde_json::error::Error as SerdeError;
use hyper::status::StatusCode;
use hyper::Error as HyperError;
use types::ClientError;

#[derive(Debug)]
pub enum StashError {
    Serialization(SerdeError),
    Http(HyperError),
    IO(IoError),
    Other(String),
    Client {
        code: StatusCode,
        error: ClientError
    }
}

impl StdError for StashError {
    fn description(&self) -> &str {
        match *self {
            StashError::Serialization(ref e) => e.description(),
            StashError::Http(ref e) => e.description(),
            StashError::IO(ref e) => e.description(),
            StashError::Other(ref e) => &e[..],
            StashError::Client { ref error, .. } => {
                if error.errors.len() == 1 {
                    &error.errors[0].message
                } else {
                    "Multiple errors encountered"
                }
            }
        }
    }
    
    fn cause(&self) -> Option<&StdError> {
        match *self {
            StashError::Serialization(ref e) => Some(e),
            StashError::Http(ref e) => Some(e),
            StashError::IO(ref e) => Some(e),
            _ => None
        }
    }
}

impl ::std::fmt::Display for StashError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! wrap_error {
    ($error_type:ident, $enum_val:ident) => {
        impl From<$error_type> for StashError {
            fn from(error: $error_type) -> StashError {
                StashError::$enum_val(error)
            }
        }
    }
}

wrap_error!(SerdeError, Serialization);
wrap_error!(HyperError, Http);
wrap_error!(IoError, IO);