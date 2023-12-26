use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::parser::CommandResponse;

#[derive(Debug)]
pub enum QueryError {
    ConnectionClosed,

    // wrapper
    MalformedUTF8(std::str::Utf8Error),
    ConnectionFailed(std::io::Error),
    ReadError(std::io::Error),
    WriteError(std::io::Error),
    FormatError(std::fmt::Error),

    // response parser
    MissingName { response: String },
    MissingKey { response: String, key: String },

    // response getters
    MissingArg { key: String },
    ArgTypeError { key: String, value: String, expected_type: String, error: String },

    // other
    MalformedEscapeSequence { src: String },
    NotTS3Server,
    UnknownKey { response: String, key: String },
    UnknownEvent { response: String, event: String },
    InvalidArgument { name: String, message: String },

    QueryError { id: i32, message: String, response: CommandResponse }
}

impl Error for QueryError {}

impl Display for QueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::ConnectionClosed => write!(f, "Connection closed"),

            QueryError::MalformedUTF8(e) => write!(f, "Malformed UTF-8: {}", e),
            QueryError::ConnectionFailed(e) => write!(f, "Connection failed: {}", e),
            QueryError::ReadError(e) => write!(f, "Read error: {}", e),
            QueryError::WriteError(e) => write!(f, "Write error: {}", e),
            QueryError::FormatError(e) => write!(f, "Format error: {}", e),

            QueryError::MissingName { response } => write!(f, "Missing name: {}", response),
            QueryError::MissingKey { response, key } => write!(f, "Missing key: {} in {}", key, response),

            QueryError::MissingArg { key } => write!(f, "Missing argument: {}", key),
            QueryError::ArgTypeError { key, value, expected_type, error } => write!(f, "Argument type error: {}={} (expected {}, got {})", key, value, expected_type, error),

            QueryError::MalformedEscapeSequence { src } => write!(f, "Malformed escape sequence: {}", src),
            QueryError::NotTS3Server => write!(f, "Not a TS3 server"),
            QueryError::UnknownKey { response, key } => write!(f, "Unknown key: {} in {}", key, response),
            QueryError::UnknownEvent { response, event } => write!(f, "Unknown event: {} in {}", event, response),
            QueryError::InvalidArgument { name, message } => write!(f, "Invalid argument: {} ({})", name, message),

            QueryError::QueryError { id, message, response } => write!(f, "Query error: {} ({}) in {}", message, id, response)
        }
    }
}
