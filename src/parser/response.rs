use std::collections::HashMap;
use std::fmt::Display;
use log::warn;
use crate::error::QueryError;
use crate::parser::util::unescape;

#[derive(Debug)]
pub struct CommandResponse {
    pub name: Option<String>,
    pub args: HashMap<String, String>,
}

// getters
impl CommandResponse {
    pub fn get(&mut self, key: &str) -> Result<String, QueryError> {
        self.args.remove(key).ok_or_else(|| QueryError::MissingArg {
            key: key.to_string(),
        })
    }

    pub fn get_i32(&mut self, key: &str) -> Result<i32, QueryError> {
        let val = self.get(key)?;

        val.parse::<i32>()
            .map_err(|e| QueryError::ArgTypeError {
                key: key.to_string(),
                value: val,
                expected_type: "integer".to_string(),
                error: e.to_string(),
            })
    }

    pub fn get_bool(&mut self, key: &str) -> Result<bool, QueryError> {
        Ok(self.get_i32(key)? != 0)
    }

    #[allow(dead_code)]
    pub fn get_opt(&mut self, key: &str) -> Option<String> {
        self.args.remove(key)
    }

    #[allow(dead_code)]
    pub fn get_opt_i32(&mut self, key: &str) -> Result<Option<i32>, QueryError> {
        if let Some(val) = self.get_opt(key) {
            Ok(Some(Self::parse_to_i32(key, &val)?))
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    pub fn get_opt_bool(&mut self, key: &str) -> Result<Option<bool>, QueryError> {
        self.get_opt_i32(key)
            .map(|val| val.map(|val| val != 0))
    }

    pub fn get_list(&mut self, key: &str) -> Result<Vec<String>, QueryError> {
        let mut list = Vec::new();
        let val = self.get(key)?;

        for val in val.split(',') {
            list.push(val.to_string());
        }

        Ok(list)
    }

    pub fn get_i32_list(&mut self, key: &str) -> Result<Vec<i32>, QueryError> {
        let mut list = Vec::new();
        let val = self.get(key)?;

        for val in val.split(',') {
            list.push(Self::parse_to_i32(key, val)?);
        }

        Ok(list)
    }

    // Only for debugging purposes to prevent Drop from logging warnings
    pub(crate) fn clear(&mut self) {
        self.args.clear();
    }

    fn parse_to_i32(key: &str, value: &str) -> Result<i32, QueryError> {
        value.parse::<i32>()
            .map_err(|e| QueryError::ArgTypeError {
                key: key.to_string(),
                value: value.to_string(),
                expected_type: "integer".to_string(),
                error: e.to_string(),
            })
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
                let mut result = String::new();

                unescape(val, &mut result)?;
                args.insert(key.to_owned(), result);
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
            warn!("Missing {} with value {} in \"{}\"", key, val, self);
        }
    }
}