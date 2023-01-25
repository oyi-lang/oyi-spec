use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct ReadError {
    message: String,
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ReadError {
    pub fn new(message: String) -> Self {
        ReadError { message }
    }
}

impl Error for ReadError {}

impl From<io::Error> for ReadError {
    fn from(io_err: io::Error) -> Self {
        ReadError {
            message: io_err.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct WriteError {
    message: String,
}

impl WriteError {
    pub fn new(message: String) -> Self {
        WriteError { message }
    }
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WriteError {}

impl From<io::Error> for WriteError {
    fn from(io_err: io::Error) -> Self {
        WriteError {
            message: io_err.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SerializationError;

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "SerializationError")
    }
}

impl Error for SerializationError {}

#[derive(Debug)]
pub struct DeserializationError;

impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "DeserializationError")
    }
}

impl Error for DeserializationError {}
