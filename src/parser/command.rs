use std::fmt::{Arguments, Display, Write};
use crate::error::QueryError;
use crate::parser::util::escape;
use crate::properties::PropertyType;

pub struct Command {
    pub buf: String,
}

#[allow(dead_code)]
impl Command {
    pub fn new(name: &str) -> Self {
        Self {
            buf: name.into(),
        }
    }

    pub fn action(mut self, key: &str) -> Self {
        self.buf.push(' ');
        self.buf.push('-');
        self.buf.push_str(key);
        self
    }

    pub fn flag(mut self, key: &str, state: bool) -> Self {
        if state {
            self.buf.push(' ');
            self.buf.push('-');
            self.buf.push_str(key);
        }
        self
    }

    pub fn key_val_str(mut self, key: &str, val: &str) -> Self {
        self.buf.push(' ');
        self.buf.push_str(key);
        self.buf.push('=');
        escape(val, &mut self.buf);
        self
    }

    pub fn key_val_i32(mut self, key: &str, val: i32) -> Result<Self, QueryError> {
        self.buf.push(' ');
        self.buf.push_str(key);
        self.buf.push('=');
        write!(self.buf, "{}", val).map_err(QueryError::FormatError)?;
        Ok(self)
    }

    pub fn key_val_bool(mut self, key: &str, val: bool) -> Self {
        self.buf.push(' ');
        self.buf.push_str(key);
        self.buf.push('=');
        if val {
            self.buf.push('1');
        } else {
            self.buf.push('0');
        }
        self
    }

    pub fn key_val_property(self, key: &str, val: PropertyType) -> Result<Self, QueryError> {
        Ok(match val {
            PropertyType::Str(val) => self.key_val_str(key, val),
            PropertyType::Int(val) => self.key_val_i32(key, val)?,
            PropertyType::Bool(val) => self.key_val_bool(key, val),
        })
    }

    pub fn key_val_i32_list(self, key: &str, val: &[i32]) -> Result<Self, QueryError> {
        self.key_val_list(key, val, |buf, e| {
            write!(buf, "{}", e)
        }).map_err(QueryError::FormatError)
    }

    pub fn key_val_str_list(self, key: &str, val: &[&str]) -> Self {
        self.key_val_list(key, val, |buf, e| {
            escape(e, buf);
            Ok(())
        }).unwrap()
    }

    pub fn raw(mut self, val: &str) -> Self {
        self.buf.push(' ');
        self.buf.push_str(val);
        self
    }

    pub fn into(self) -> String {
        self.buf
    }

    #[inline(always)]
    fn key_val_list<F, T>(
        mut self,
        key: &str,
        val: &[T],
        write_value: F
    ) -> Result<Self, std::fmt::Error>
        where F: Fn(&mut String, &T) -> std::fmt::Result
    {
        let Some(first) = val.first() else {
            return Ok(self);
        };

        self.buf.push(' ');
        self.buf.push_str(key);
        self.buf.push('=');
        write_value(&mut self.buf, first)?;

        for val in val.iter().skip(1) {
            self.buf.push('|');
            self.buf.push_str(key);
            self.buf.push('=');
            write_value(&mut self.buf, val)?;
        }

        Ok(self)
    }

}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buf)?;
        Ok(())
    }
}

impl Write for Command {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.push_str(s);
        Ok(())
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> std::fmt::Result {
        self.buf.write_fmt(args)?;
        Ok(())
    }
}
