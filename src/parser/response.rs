use std::collections::HashMap;
use std::fmt::Display;
use log::warn;
use crate::error::QueryError;
use crate::parser::escape::unescape;

#[derive(Debug)]
pub struct CommandResponse {
    pub name: Option<String>,
    pub args: HashMap<String, String>,
}

// getters
impl CommandResponse {
    pub fn get<D: Decode + Default>(&mut self, key: &str) -> Result<D, QueryError> {
        self.get_or(key, D::default)
    }

    pub fn get_or<D: Decode, F: FnOnce() -> D>(&mut self, key: &str, default: F) -> Result<D, QueryError> {
        match self.args.remove(key) {
            Some(val) => D::decode(key, val),
            None => Ok(default()),
        }
    }
}

// decoder
impl CommandResponse {
    pub fn decode(buf: &str, parse_name: bool) -> Result<Self, QueryError> {
        let mut parts = buf.split(' ');

        let name = if parse_name {
            Some(
                parts.next()
                    .ok_or_else(|| QueryError::MissingName { response: buf.to_string() })?
                    .to_string()
            )
        } else {
            None
        };

        let mut args = HashMap::new();

        for arg in parts {
            let mut parts = arg.splitn(2, '=');
            let key = parts.next()
                .ok_or_else(|| QueryError::MissingKey {
                    response: buf.to_string(),
                    key: arg.to_string(),
                })?;
            let val = parts.next();

            if let Some(val) = val {
                if val.contains('|') {
                    let mut buf = String::new();

                    for arg in arg.split('|') {
                        let mut parts = arg.splitn(2, '=');
                        let sub_key = parts.next()
                            .ok_or_else(|| QueryError::MissingKey {
                                response: buf.to_string(),
                                key: arg.to_string(),
                            })?;

                        if sub_key != key {
                            return Err(QueryError::InvalidArgument {
                                name: sub_key.to_string(),
                                message: format!("'{}' in multi-arg response does not match the first key '{}'", sub_key, key),
                            });
                        }

                        let val = parts.next();

                        if let Some(val) = val {
                            if !buf.is_empty() {
                                buf.push(',');
                            }

                            unescape(val, &mut buf)?;
                        } else {
                            return Err(QueryError::InvalidArgument {
                                name: sub_key.to_string(),
                                message: "Missing value".to_string(),
                            });
                        }
                    }

                    args.insert(key.to_owned(), buf);
                } else {
                    let mut result = String::new();

                    unescape(val, &mut result)?;
                    args.insert(key.to_owned(), result);
                }
            } else {
                args.insert(key.to_owned(), String::new());
            }
        }

        Ok(Self {
            name,
            args,
        })
    }

    pub fn decode_multi(buf: &str) -> Result<Vec<Self>, QueryError> {
        let mut responses = Vec::new();

        for buf in buf.split('|') {
            responses.push(Self::decode(buf, false)?);
        }

        Ok(responses)
    }
}

impl Display for CommandResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{} ", name)?;
        }

        let mut first = true;

        for (key, val) in &self.args {
            if first {
                first = false;
            } else {
                write!(f, " ")?;
            }

            write!(f, "{}={}", key, val)?;
        }

        Ok(())
    }
}

impl Drop for CommandResponse {
    fn drop(&mut self) {
        // Only for debugging if stuff is missing
        for (key, val) in &self.args {
            if key == "msg" {
                continue;
            }

            warn!("Missing {} with value {} in \"{}\"", key, val, self);
        }
    }
}

pub trait Decode: Sized {
    fn decode(key: &str, value: String) -> Result<Self, QueryError>;
}

// placeholder
impl<T : Decode> Decode for Option<T> {
    fn decode(_key: &str, value: String) -> Result<Self, QueryError> {
        if value.is_empty() {
            return Ok(None);
        }

        Ok(Some(T::decode(_key, value)?))
    }
}

impl Decode for String {
    fn decode(_key: &str, value: String) -> Result<Self, QueryError> {
        Ok(value)
    }
}

impl Decode for bool {
    fn decode(_key: &str, value: String) -> Result<Self, QueryError> {
        match value.as_str() {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => Err(QueryError::ArgTypeError {
                key: _key.to_string(),
                value,
                expected_type: "boolean".to_string(),
                error: "Invalid boolean value".to_string(),
            })
        }
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(_key: &str, value: String) -> Result<Self, QueryError> {
        let mut list = Vec::new();

        for val in value.split(',') {
            list.push(T::decode(_key, val.to_string())?);
        }

        Ok(list)
    }
}

macro_rules! impl_decode {
    ($($type:ident),*) => {
        $(
            impl Decode for $type {
                fn decode(_key: &str, value: String) -> Result<Self, QueryError> {
                    value.parse::<$type>()
                        .map_err(|e| QueryError::ArgTypeError {
                            key: _key.to_string(),
                            value,
                            expected_type: stringify!($type).to_string(),
                            error: e.to_string(),
                        })
                }
            }
        )*
    };
}

impl_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_none() {
        let response = CommandResponse::decode("test", true).unwrap();

        assert_eq!(response.name, Some("test".to_string()));
        assert_eq!(response.args.len(), 0);
    }

    #[test]
    fn test_decode_str() {
        let mut response = CommandResponse::decode("test some_string=hello", true).unwrap();

        assert_eq!(response.name, Some("test".to_string()));

        match response.get::<String>("some_string") {
            Ok(val) => assert_eq!(val, "hello"),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);
    }

    #[test]
    fn test_decode_i32() {
        let mut response = CommandResponse::decode("test some_integer=69", true).unwrap();

        assert_eq!(response.name, Some("test".to_string()));

        match response.get::<i32>("some_integer") {
            Ok(val) => assert_eq!(val, 69),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_decode_bool() {
        let mut response = CommandResponse::decode("test some_bool=1", true).unwrap();

        assert_eq!(response.name, Some("test".to_string()));

        match response.get::<bool>("some_bool") {
            Ok(val) => assert_eq!(val, true),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);
    }

    #[test]
    fn test_decode_list() {
        let mut response = CommandResponse::decode("test some_list=hello,world", true).unwrap();

        assert_eq!(response.name, Some("test".to_string()));

        match response.get::<Vec<String>>("some_list") {
            Ok(val) => assert_eq!(val, vec!["hello", "world"]),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);
    }

    #[test]
    fn test_decode_i32_list() {
        let mut response = CommandResponse::decode("test some_list=69,420", true).unwrap();

        assert_eq!(response.name, Some("test".to_string()));

        match response.get::<Vec<i32>>("some_list") {
            Ok(val) => assert_eq!(val, vec![69, 420]),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);
    }

    #[test]
    fn test_decode_str_without_name() {
        let mut response = CommandResponse::decode("some_string=hello", false).unwrap();

        assert_eq!(response.name, None);

        match response.get::<String>("some_string") {
            Ok(val) => assert_eq!(val, "hello"),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);
    }

    #[test]
    fn test_decode_multi_but_only_one() {
        let mut responses = CommandResponse::decode_multi("test1").unwrap();

        assert_eq!(responses.len(), 1);

        let mut response = responses.remove(0);

        assert_eq!(response.name, None);
        assert_eq!(response.args.len(), 1);

        match response.get::<String>("test1") {
            Ok(val) => assert_eq!(val, ""),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);
    }

    #[test]
    fn test_decode_multi() {
        let mut responses = CommandResponse::decode_multi("test1=hi|test2=mom").unwrap();

        assert_eq!(responses.len(), 2);

        let mut response = responses.remove(0);

        assert_eq!(response.name, None);
        assert_eq!(response.args.len(), 1);

        match response.get::<String>("test1") {
            Ok(val) => assert_eq!(val, "hi"),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);

        let mut response = responses.remove(0);

        assert_eq!(response.name, None);
        assert_eq!(response.args.len(), 1);

        match response.get::<String>("test2") {
            Ok(val) => assert_eq!(val, "mom"),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);
    }

    #[test]
    fn test_decode_multi_multiple_args() {
        let mut responses = CommandResponse::decode_multi("test1=hi test2=69|test1=mom test2=420").unwrap();

        assert_eq!(responses.len(), 2);

        let mut response = responses.remove(0);

        assert_eq!(response.name, None);
        assert_eq!(response.args.len(), 2);

        match response.get::<String>("test1") {
            Ok(val) => assert_eq!(val, "hi"),
            Err(e) => panic!("{:?}", e),
        }

        match response.get::<String>("test2") {
            Ok(val) => assert_eq!(val, "69"),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);

        let mut response = responses.remove(0);

        assert_eq!(response.name, None);
        assert_eq!(response.args.len(), 2);

        match response.get::<String>("test1") {
            Ok(val) => assert_eq!(val, "mom"),
            Err(e) => panic!("{:?}", e),
        }

        match response.get::<String>("test2") {
            Ok(val) => assert_eq!(val, "420"),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(response.args.len(), 0);
    }

}