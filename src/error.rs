use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum DeltaError {
    UnknownScene
}

impl fmt::Display for DeltaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeltaError")
    }
}

impl Error for DeltaError {
    fn description(&self) -> &str {
        match *self {
            DeltaError::UnknownScene => "Cannot set the current scene"
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            DeltaError::UnknownScene => None
        }
    }
}
