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
pub struct SerializeError {
    message: String,
}

impl SerializeError {
    pub fn new(message: String) -> Self {
        SerializeError { message }
    }
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SerializeError {}

impl From<WriteError> for SerializeError {
    fn from(write_err: WriteError) -> Self {
        SerializeError {
            message: write_err.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct DeserializeError {
    message: String,
}

impl DeserializeError {
    pub fn new(message: String) -> Self {
        DeserializeError { message }
    }
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DeserializeError {}

impl From<ReadError> for DeserializeError {
    fn from(read_err: ReadError) -> Self {
        DeserializeError {
            message: read_err.to_string(),
        }
    }
}
