use crate::error::ParseError;
use crate::parser::escape::escape;
use std::fmt::{Arguments, Display, Write};

pub struct Command {
    pub buf: String,
    pub prepend_space: bool,
}

#[allow(dead_code)]
impl Command {
    pub fn new(name: &str) -> Self {
        Self {
            buf: name.into(),
            prepend_space: true,
        }
    }

    pub fn flag(mut self, key: &str, state: bool) -> Self {
        if state {
            if self.prepend_space {
                self.buf.push(' ');
            }

            self.buf.push('-');
            self.buf.push_str(key);
        }

        self
    }

    pub fn arg_opt<T: Encode>(self, key: &str, val: Option<T>) -> Result<Self, ParseError> {
        if let Some(val) = val {
            self.arg(key, val)
        } else {
            Ok(self)
        }
    }

    pub fn arg<T: Encode>(self, key: &str, val: T) -> Result<Self, ParseError> {
        self.arg_ref(key, &val)
    }

    pub fn arg_ref<T: Encode>(mut self, key: &str, val: &T) -> Result<Self, ParseError> {
        if self.prepend_space {
            self.buf.push(' ');
        }

        self.buf.push_str(key);
        self.buf.push('=');
        val.encode(&mut self.buf).map_err(ParseError::FormatError)?;

        Ok(self)
    }

    pub fn arg_list<T: Encode>(mut self, key: &str, val: &[T]) -> Result<Self, ParseError> {
        let Some(first) = val.first() else {
            return Ok(self);
        };

        self = self.arg_ref(key, first)?;

        for val in val.iter().skip(1) {
            self = self.list_sep().arg_ref(key, val)?;
        }

        self.prepend_space = true;

        Ok(self)
    }

    pub fn arg_multi_list<T: EncodeList>(mut self, val: &[T]) -> Result<Self, ParseError> {
        if self.prepend_space {
            self.buf.push(' ');
        }

        let Some(first) = val.first() else {
            return Ok(self);
        };

        first.encode_list(&mut CommandListBuilder::new(&mut self.buf))?;

        for val in val.iter().skip(1) {
            self = self.list_sep();

            val.encode_list(&mut CommandListBuilder::new(&mut self.buf))?;
        }

        self.prepend_space = true;

        Ok(self)
    }

    pub fn list_sep(mut self) -> Self {
        self.prepend_space = false;
        self.buf.push('|');
        self
    }

    pub fn raw(mut self, val: &str) -> Self {
        if self.prepend_space {
            self.buf.push(' ');
        }

        self.buf.push_str(val);
        self
    }
}

impl From<Command> for String {
    fn from(command: Command) -> Self {
        command.buf
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

pub trait Encode {
    fn encode(&self, buf: &mut String) -> std::fmt::Result;
}

impl Encode for &str {
    fn encode(&self, buf: &mut String) -> std::fmt::Result {
        escape(self, buf);
        Ok(())
    }
}

impl Encode for String {
    fn encode(&self, buf: &mut String) -> std::fmt::Result {
        escape(self, buf);
        Ok(())
    }
}

impl Encode for bool {
    fn encode(&self, buf: &mut String) -> std::fmt::Result {
        if *self {
            buf.push('1');
        } else {
            buf.push('0');
        }
        Ok(())
    }
}

macro_rules! impl_simple_encode {
    ($($t:ty),*) => {
        $(
            impl Encode for $t {
                fn encode(&self, buf: &mut String) -> std::fmt::Result {
                    write!(buf, "{}", self)
                }
            }
        )*
    };
}

impl_simple_encode!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

pub struct CommandListBuilder<'a> {
    buf: &'a mut String,
    prepend_space: bool,
}

impl<'a> CommandListBuilder<'a> {
    pub fn new(buf: &'a mut String) -> Self {
        Self {
            buf,
            prepend_space: false,
        }
    }

    pub fn add<T: Encode>(&mut self, key: &str, val: T) -> Result<(), ParseError> {
        self.add_ref(key, &val)
    }

    pub fn add_ref<T: Encode>(&mut self, key: &str, val: &T) -> Result<(), ParseError> {
        if self.prepend_space {
            self.buf.push(' ');
        } else {
            self.prepend_space = true;
        }

        self.buf.push_str(key);
        self.buf.push('=');
        val.encode(self.buf).map_err(ParseError::FormatError)?;

        Ok(())
    }
}

pub trait EncodeList {
    fn encode_list(&self, builder: &mut CommandListBuilder) -> Result<(), ParseError>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct Test {
        key: &'static str,
        key2: &'static str,
    }

    impl EncodeList for Test {
        fn encode_list(&self, builder: &mut CommandListBuilder) -> Result<(), ParseError> {
            builder.add("key", self.key)?;
            builder.add("key2", self.key2)
        }
    }

    #[test]
    fn test_arg() {
        let command = Command::new("test")
            .arg("key", "value")
            .unwrap()
            .arg("key2", 123)
            .unwrap()
            .arg("key3", true)
            .unwrap()
            .arg("key4", false)
            .unwrap();

        let command: String = command.into();

        assert_eq!(command, "test key=value key2=123 key3=1 key4=0")
    }

    #[test]
    fn test_arg_list_single() {
        let command = Command::new("test")
            .arg_list("key", &["value1"])
            .unwrap()
            .arg_list("key2", &[123])
            .unwrap();

        let command: String = command.into();

        assert_eq!(command, "test key=value1 key2=123")
    }

    #[test]
    fn test_arg_list() {
        let command = Command::new("test")
            .arg_list("key", &["value1", "value2", "value3"])
            .unwrap()
            .arg_list("key2", &[123, 456, 789])
            .unwrap();

        let command: String = command.into();

        assert_eq!(
            command,
            "test key=value1|key=value2|key=value3 key2=123|key2=456|key2=789"
        )
    }

    #[test]
    fn test_arg_multi_list() {
        let command = Command::new("test")
            .arg_multi_list(&[
                Test {
                    key: "value1",
                    key2: "value2",
                },
                Test {
                    key: "value3",
                    key2: "value4",
                },
                Test {
                    key: "value5",
                    key2: "value6",
                },
            ])
            .unwrap();

        let command: String = command.into();

        assert_eq!(
            command,
            "test key=value1 key2=value2|key=value3 key2=value4|key=value5 key2=value6"
        )
    }

    #[test]
    fn test_arg_multi_list_single() {
        let command = Command::new("test")
            .arg_multi_list(&[Test {
                key: "value1",
                key2: "value2",
            }])
            .unwrap();

        let command: String = command.into();

        assert_eq!(command, "test key=value1 key2=value2")
    }
}
