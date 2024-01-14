use crate::parser::Separator;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    // wrapper
    #[error("Format error: {0}")]
    FormatError(std::fmt::Error),

    // token parse errors
    #[error("Format error: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("Format error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Error while parsing int: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Error while parsing float: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("Error while parsing bool: {0}")]
    ParseBool(#[from] std::str::ParseBoolError),
    #[error("Error while parsing bool: {0} (expected 0 or 1)")]
    ParseIntBool(Cow<'static, str>),

    // decode errors
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("Malformed escape sequence: {src}")]
    MalformedEscapeSequence { src: String },
    #[error("Invalid Separator")]
    InvalidSeparator(Separator),
    #[error("Missing {0}")]
    MissingKey(String),
    #[error("Invalid key: {0}")]
    ValueParseError(String, Box<ParseError>),
    #[error("Missing value for {0}")]
    MissingValue(String),
    #[error("No scope")]
    NoScope,
    #[error("End of file")]
    Eof,

    #[error("Unknown permission: {id}")]
    UnknownPermission { id: String },
    #[error("Unknown channel property: {id}")]
    UnknownChannelProperty { id: String },
    #[error("Unknown event: {event} in {response}")]
    UnknownEvent { response: String, event: String },

    #[error("Invalid Value: {0}")]
    InvalidValue(Cow<'static, str>),

    #[error("{0}")]
    Other(Cow<'static, str>),
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Connection closed")]
    ConnectionClosed,

    // wrapper
    #[error("Malformed UTF-8: {0}")]
    MalformedUTF8(#[from] std::str::Utf8Error),
    #[error("Malformed UTF-8: {0}")]
    MalformedUTF8String(#[from] std::string::FromUtf8Error),
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
    #[error("Invalid argument '{name}': {message}")]
    InvalidArgument {
        name: &'static str,
        message: Cow<'static, str>,
    },

    #[error("Query error: {message} ({id})")]
    QueryError { id: i32, message: String },
}
