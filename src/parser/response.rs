use crate::error::ParseError;
use crate::parser::escape::unescape;
use log::{log_enabled, warn};
use std::borrow::Cow;

pub struct Decoder<'a> {
    buf: &'a [u8],
    pos: usize,
    cur_sep: Separator,
    entries: Vec<Vec<Pair>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Separator {
    Assign,
    Pair,
    List,
    Eof,
}

pub struct Pair {
    pub key: String,
    pub value: Option<String>,
}

impl<'a> Decoder<'a> {
    const ASSIGN: u8 = b'=';
    const PAIR_SEP: u8 = b' ';
    const LIST_SEP: u8 = b'|';

    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            cur_sep: Separator::Eof,
            entries: vec![Vec::new()],
        }
    }

    pub fn decode<T>(&mut self) -> Result<T, ParseError>
    where
        T: Decode,
    {
        T::decode(self)
    }

    pub fn decode_name(&mut self) -> Result<String, ParseError> {
        let key = self.parse_key()?;

        self.parse_sep()?;

        Ok(key)
    }

    pub fn decode_with_name<T>(&mut self) -> Result<T, ParseError>
    where
        T: Decode,
    {
        let _ = self.decode_name()?;
        let value = T::decode(self)?;

        Ok(value)
    }

    pub fn push_scope(&mut self) {
        self.entries.push(Vec::new());
    }

    pub fn pop_scope(&mut self) {
        if let Some(missing) = self.entries.pop() {
            if log_enabled!(log::Level::Debug) {
                for pair in missing {
                    warn!("Missing key: '{}'", pair.key);
                }
            }
        }
    }

    pub fn last_sep(&self) -> Separator {
        self.cur_sep
    }

    pub fn advance<T>(&mut self, key: &'a str) -> Result<Option<T>, ParseError>
    where
        T: DecodeValue,
    {
        match self.advance_internal(key) {
            Ok(Some(value)) => T::decode(key, value).map(Some),
            Ok(None) => Ok(None),
            Err(ParseError::Eof) => Err(ParseError::MissingKey(key.to_string())),
            Err(e) => Err(e),
        }
    }

    pub fn advance_or_none<T>(&mut self, key: &'a str) -> Result<Option<T>, ParseError>
    where
        T: DecodeValue,
    {
        match self.advance_internal(key) {
            Ok(Some(value)) => T::decode(key, value).map(Some),
            Ok(None) | Err(ParseError::Eof) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn advance_or_default<T>(&mut self, key: &'a str) -> Result<T, ParseError>
    where
        T: DecodeValue + Default,
    {
        self.advance_or_none(key).map(|v| v.unwrap_or_default())
    }

    pub fn advance_or_err<T>(&mut self, key: &'a str) -> Result<T, ParseError>
    where
        T: DecodeValue,
    {
        match self.advance(key) {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Err(ParseError::MissingValue(key.to_owned())),
            Err(e) => Err(e),
        }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.buf.len()
    }

    fn parse_key(&mut self) -> Result<String, ParseError> {
        let pos = self.pos;
        let len = self.buf.len();

        let end = self.buf[pos..]
            .iter()
            .position(|&b| b == Self::ASSIGN || b == Self::PAIR_SEP || b == Self::LIST_SEP)
            .map(|cur_pos| pos + cur_pos)
            .unwrap_or(len);

        let vec = Vec::from(&self.buf[pos..end]);
        let key = String::from_utf8(vec)?;

        self.pos = end;

        Ok(key)
    }

    fn parse_sep(&mut self) -> Result<(), ParseError> {
        let pos = self.pos;
        let len = self.buf.len();

        if pos >= len {
            self.cur_sep = Separator::Eof;
            return Ok(());
        }

        self.cur_sep = match self.buf[pos] {
            Self::ASSIGN => {
                self.pos += 1;
                Separator::Assign
            }
            Self::PAIR_SEP => {
                self.pos += 1;
                Separator::Pair
            }
            Self::LIST_SEP => {
                self.pos += 1;
                Separator::List
            }
            _ => {
                return Err(ParseError::UnexpectedToken(
                    (self.buf[pos] as char).to_string(),
                ))
            }
        };

        Ok(())
    }

    fn parse_value(&mut self) -> Result<String, ParseError> {
        let pos = self.pos;
        let len = self.buf.len();

        let end = self.buf[pos..]
            .iter()
            .position(|&b| b == Self::PAIR_SEP || b == Self::LIST_SEP)
            .map(|cur_pos| pos + cur_pos)
            .unwrap_or(len);

        let mut value = String::with_capacity(end - pos);

        unescape(&self.buf[pos..end], &mut value)?;

        self.pos = end;

        Ok(value)
    }

    fn parse_pair(&mut self) -> Result<Pair, ParseError> {
        let key = self.parse_key()?;

        self.parse_sep()?;

        match self.cur_sep {
            Separator::Assign => {
                let value = self.parse_value()?;

                self.parse_sep()?;

                Ok(Pair {
                    key,
                    value: Some(value),
                })
            }
            _ => Ok(Pair { key, value: None }),
        }
    }

    fn advance_internal(&mut self, key: &'a str) -> Result<Option<String>, ParseError> {
        let scope = self.entries.last_mut().ok_or(ParseError::NoScope)?;

        if let Some(i) = scope.iter().position(|k| k.key == key) {
            return Ok(scope.remove(i).value);
        }

        if self.is_eof() {
            return Err(ParseError::Eof);
        }

        loop {
            let pair = self.parse_pair()?;

            if pair.key == key {
                return Ok(pair.value);
            } else {
                self.entries
                    .last_mut()
                    .ok_or(ParseError::NoScope)?
                    .push(pair);
            }

            match self.cur_sep {
                Separator::Pair => {}
                Separator::Eof => return Err(ParseError::Eof),
                _ => return Err(ParseError::InvalidSeparator(self.cur_sep)),
            }
        }
    }
}

impl Drop for Decoder<'_> {
    fn drop(&mut self) {
        self.pop_scope()
    }
}

pub trait Decode {
    fn decode(decoder: &mut Decoder) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait DecodeValue {
    fn decode(key: &str, value: String) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait DecodeInto {
    fn decode_into(self, decoder: &mut Decoder) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait DecodeCustomInto<T> {
    fn decode_into<F>(self, decoder: &mut Decoder, gen: F) -> Result<Self, ParseError>
    where
        F: Fn(&mut Decoder) -> Result<T, ParseError>,
        Self: Sized;
}

impl DecodeValue for String {
    fn decode(_key: &str, value: String) -> Result<Self, ParseError> {
        Ok(value)
    }
}

impl DecodeValue for bool {
    fn decode(_key: &str, value: String) -> Result<Self, ParseError> {
        match value.as_ref() {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => Err(ParseError::ParseIntBool(Cow::from(value))),
        }
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(decoder: &mut Decoder) -> Result<Self, ParseError> {
        decoder.push_scope();

        let mut vec = Vec::new();

        loop {
            vec.push(T::decode(decoder)?);

            match decoder.last_sep() {
                Separator::List => {}
                Separator::Eof => break,
                _ => return Err(ParseError::InvalidSeparator(decoder.last_sep())),
            }
        }

        decoder.pop_scope();

        Ok(vec)
    }
}

impl<T: Decode> DecodeInto for Vec<T> {
    fn decode_into(mut self, decoder: &mut Decoder) -> Result<Self, ParseError> {
        decoder.push_scope();

        loop {
            self.push(T::decode(decoder)?);

            match decoder.last_sep() {
                Separator::List => {}
                Separator::Eof => break,
                _ => return Err(ParseError::InvalidSeparator(decoder.last_sep())),
            }
        }

        decoder.pop_scope();

        Ok(self)
    }
}

impl<T> DecodeCustomInto<T> for Vec<T> {
    fn decode_into<F>(mut self, decoder: &mut Decoder, gen: F) -> Result<Self, ParseError>
    where
        F: Fn(&mut Decoder) -> Result<T, ParseError>,
    {
        decoder.push_scope();

        loop {
            self.push(gen(decoder)?);

            match decoder.last_sep() {
                Separator::List => {}
                Separator::Eof => break,
                _ => return Err(ParseError::InvalidSeparator(decoder.last_sep())),
            }
        }

        decoder.pop_scope();

        Ok(self)
    }
}

impl<T: DecodeValue> DecodeValue for Vec<T> {
    fn decode(key: &str, value: String) -> Result<Self, ParseError> {
        let mut vec = Vec::new();

        for val in value.split(',') {
            vec.push(T::decode(key, val.to_owned())?);
        }

        Ok(vec)
    }
}

impl<T: DecodeValue> DecodeValue for Option<T> {
    fn decode(key: &str, value: String) -> Result<Self, ParseError> {
        if value.is_empty() {
            Ok(None)
        } else {
            Ok(Some(T::decode(key, value)?))
        }
    }
}

macro_rules! impl_decode {
    ($($type:ident),*) => {
        $(
            impl DecodeValue for $type {
                fn decode(_key: &str, value: String) -> Result<Self, ParseError> {
                    Ok(value.parse::<$type>()?)
                }
            }
        )*
    };
}

impl_decode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

#[cfg(test)]
mod test {
    use super::*;
    use crate::macros::ts_response;

    #[test]
    fn test_decode_name() {
        let response = Decoder::new(b"test").decode_name().unwrap();

        assert_eq!(response, "test");
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_decode_struct() {
        ts_response! {
            Test {
                some_string: String,
                some_integer: i32,
                some_bool: bool,
                some_list: Vec<String>,
                some_list2: Vec<i32>,
            }
        }

        let mut decoder = Decoder::new(b"some_string=hello some_integer=69 some_bool=1 some_list=hello,world some_list2=69,420");
        let response = Test::decode(&mut decoder).unwrap();

        assert_eq!(response.some_string, "hello");
        assert_eq!(response.some_integer, 69);
        assert_eq!(response.some_bool, true);
        assert_eq!(response.some_list, vec!["hello", "world"]);
        assert_eq!(response.some_list2, vec![69, 420]);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_decode_struct_reverse_order() {
        ts_response! {
            Test {
                some_string: String,
                some_integer: i32,
                some_bool: bool,
                some_list: Vec<String>,
                some_list2: Vec<i32>,
            }
        }

        let mut decoder = Decoder::new(b"some_list2=69,420 some_list=hello,world some_bool=1 some_integer=69 some_string=hello");
        let response = Test::decode(&mut decoder).unwrap();

        assert_eq!(response.some_string, "hello");
        assert_eq!(response.some_integer, 69);
        assert_eq!(response.some_bool, true);
        assert_eq!(response.some_list, vec!["hello", "world"]);
        assert_eq!(response.some_list2, vec![69, 420]);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_decode_struct_with_unknown() {
        ts_response! {
            Test {
                some_string: String,
                some_integer: i32,
                some_bool: bool,
                some_list: Vec<String>,
                some_list2: Vec<i32>,
            }
        }

        let mut decoder = Decoder::new(b"some_string=hello some_integer=69 some_bool=1 some_list=hello,world some_list2=69,420 some_unknown=hello");
        let response = Test::decode(&mut decoder).unwrap();

        assert_eq!(response.some_string, "hello");
        assert_eq!(response.some_integer, 69);
        assert_eq!(response.some_bool, true);
        assert_eq!(response.some_list, vec!["hello", "world"]);
        assert_eq!(response.some_list2, vec![69, 420]);
    }

    #[test]
    fn test_decode_struct_with_missing() {
        ts_response! {
            Test {
                some_string: String,
                some_integer: i32,
                some_bool: bool, // missing
            }
        }

        let mut decoder = Decoder::new(b"some_string=hello some_integer=69");

        match Test::decode(&mut decoder) {
            Ok(_) => panic!("Expected error"),
            Err(ParseError::MissingKey(key)) => assert_eq!(key, "some_bool"),
            Err(e) => panic!("Expected MissingKey, got {:?}", e),
        }
    }

    #[test]
    fn test_decode_struct_with_missing_value() {
        ts_response! {
            Test {
                some_string: String,
                some_integer: i32,
                some_bool: bool,
            }
        }

        let mut decoder = Decoder::new(b"some_string=hello some_integer=69 some_bool");

        match Test::decode(&mut decoder) {
            Ok(_) => panic!("Expected error"),
            Err(ParseError::MissingValue(key)) => assert_eq!(key, "some_bool"),
            Err(e) => panic!("Expected MissingValue, got {:?}", e),
        }
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_decode_struct_with_no_value() {
        ts_response! {
            Test {
                some_string: String,
                some_integer: i32,
                some_bool: bool,
            }
        }

        let mut decoder = Decoder::new(b"some_string= some_integer=69 some_bool=1");
        let response = Test::decode(&mut decoder).unwrap();

        assert_eq!(response.some_string, "");
        assert_eq!(response.some_integer, 69);
        assert_eq!(response.some_bool, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_decode_struct_inline() {
        ts_response! {
            MyInlinedTest {
                some_integer: i32,
                some_bool: bool,
            }
        }

        ts_response! {
            Test {
                some_string: String,
                some_inlined: Inline<MyInlinedTest>,
            }
        }

        let mut decoder = Decoder::new(b"some_string=hello some_integer=69 some_bool=1");
        let response = decoder.decode::<Test>().unwrap();

        assert_eq!(response.some_string, "hello");
        assert_eq!(response.some_inlined.some_integer, 69);
        assert_eq!(response.some_inlined.some_bool, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_decode_struct_inline_vec() {
        ts_response! {
            MyInlinedTest {
                some_integer: i32,
                some_bool: bool,
            }
        }

        ts_response! {
            Test {
                some_string: String,
                some_inlined: Inline<Vec, MyInlinedTest>,
            }
        }

        let mut decoder = Decoder::new(
            b"some_string=hello some_integer=69 some_bool=1|some_integer=420 some_bool=0",
        );
        let response = decoder.decode::<Test>().unwrap();

        assert_eq!(response.some_string, "hello");
        assert_eq!(response.some_inlined[0].some_integer, 69);
        assert_eq!(response.some_inlined[0].some_bool, true);
        assert_eq!(response.some_inlined[1].some_integer, 420);
        assert_eq!(response.some_inlined[1].some_bool, false);
    }
}
