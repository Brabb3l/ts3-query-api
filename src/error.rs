use crate::parser::CommandResponse;

#[derive(Debug)]
pub enum QueryError {
    ConnectionClosed,

    // wrapper
    MalformedUTF8(std::str::Utf8Error),
    ConnectionFailed(std::io::Error),
    ReadError(std::io::Error),
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

    QueryError { id: i32, message: String, response: CommandResponse }
}