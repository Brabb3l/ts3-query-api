use thiserror::Error;
use crate::parser::CommandResponse;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Format error: {0}")]
    FormatError(std::fmt::Error),

    #[error("Malformed escape sequence: {src}")]
    MalformedEscapeSequence { src: String },

    #[error("Missing name: {response}")]
    MissingName { response: String },
    #[error("Missing key: {key} in {response}")]
    MissingKey { response: String, key: String },

    #[error("Missing argument: {key}")]
    MissingArg { key: String },
    #[error("Argument type error: {key}={value} (expected {expected_type}, got {error})")]
    ArgTypeError { key: String, value: String, expected_type: String, error: String },

    #[error("Unknown key: {key} in {response}")]
    UnknownKey { response: String, key: String },
    #[error("Unknown event: {event} in {response}")]
    UnknownEvent { response: String, event: String },
    #[error("Invalid argument: {name} ({message})")]
    InvalidArgument { name: String, message: String },

    #[error("Unknown permission: {id}")]
    UnknownPermission { id: String },

    #[error("Invalid integer: {0}")]
    InvalidInteger(#[from] std::num::ParseIntError),
    #[error("Invalid boolean: {0}")]
    InvalidBoolean(#[from] std::str::ParseBoolError),
    #[error("Invalid value: {0}")]
    InvalidValue(String),
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Connection closed")]
    ConnectionClosed,

    // wrapper
    #[error("Malformed UTF-8: {0}")]
    MalformedUTF8(std::str::Utf8Error),
    #[error("Connection failed: {0}")]
    ConnectionFailed(std::io::Error),
    #[error("Read error: {0}")]
    ReadError(std::io::Error),
    #[error("Write error: {0}")]
    WriteError(std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),

    // other
    #[error("Not a TS3 server")]
    NotTS3Server,

    #[error("Query error: {message} ({id}) in {response}")]
    QueryError { id: i32, message: String, response: CommandResponse }
}
