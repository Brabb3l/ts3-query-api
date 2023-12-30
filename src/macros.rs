macro_rules! property {
    ($value_name:ident, str) => {
        PropertyType::Str($value_name)
    };
    ($value_name:ident, bool) => {
        PropertyType::Bool(*$value_name)
    };
    ($value_name:ident, u32) => {
        PropertyType::Int(*$value_name)
    };
}

macro_rules! property_type {
    (str) => {
        &'a str
    };
    (bool) => {
        bool
    };
    (u32) => {
        u32
    }
}

macro_rules! properties {
    ($type:ident {
        $($name:ident: $ty:ident = $value:expr),* $(,)?
    }) => {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $type<'a> {
            $($name($crate::macros::property_type!($ty))),*,
            Custom(&'a str, PropertyType<'a>),
        }

        #[allow(dead_code)]
        impl<'a> $type<'a> {
            pub fn contents(&'a self) -> (&'a str, PropertyType<'a>) {
                let name = match self {
                    $( $type::$name { .. } => $value, )*
                    $type::Custom(name, _) => name,
                };

                let value = match self {
                    $( $type::$name(value) => $crate::macros::property!(value, $ty), )*
                    $type::Custom(_, value) => value.clone(),
                };

                (name, value)
            }
        }
    }
}

macro_rules! permission {
    ($value_name:expr, bool) => {
        PermissionValue::Bool(*$value_name)
    };
    ($value_name:expr, i32) => {
        PermissionValue::Int(*$value_name)
    };
}

macro_rules! permission_type {
    (bool) => {
        bool
    };
    (i32) => {
        i32
    }
}

macro_rules! permission_parse {
    ($value:expr, bool) => {
        match $value {
            0 => false,
            1 => true,
            _ => return Err($crate::error::ParseError::InvalidValue(std::borrow::Cow::from($value.to_string()))),
        }
    };
    ($value:expr, i32) => {
        $value
    }
}

macro_rules! permissions {
    ($type:ident {
        $($name:ident: $ty:ident),* $(,)?
    }) => {
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $type {
            $($name($crate::macros::permission_type!($ty))),*,
            Custom(String, PermissionValue),
        }

        #[allow(dead_code)]
        impl $type {
            pub fn parse(id: &str, value: i32, error_on_unknown: bool) -> Result<$type, $crate::error::ParseError> {
                match id {
                    $( stringify!($name) => Ok($type::$name($crate::macros::permission_parse!(value, $ty))), )*
                    _ => if error_on_unknown {
                        Err($crate::error::ParseError::UnknownPermission {
                            id: id.to_string(),
                        })
                    } else {
                        Ok($type::Custom(id.to_owned(), PermissionValue::Int(value)))
                    }
                }
            }

            pub fn into_pair(&self) -> PermissionPair {
                let id = match self {
                    $( $type::$name { .. } => stringify!($name), )*
                    $type::Custom(id, _) => id,
                };

                let value = match self {
                    $( $type::$name(value) => $crate::macros::permission!(value, $ty), )*
                    $type::Custom(_, value) => value.clone(),
                };

                PermissionPair {
                    id,
                    value,
                }
            }
        }
    }
}

macro_rules! decode_key {
    ($key:ident) => { stringify!($key) };
    ($key:ident, $opt_name:expr) => { $opt_name };
}

macro_rules! decode_type {
    (Inline<$type:tt$(, $sg:tt $(, $sg2:tt)?)?>) => { $type $(<$sg$(<$sg2>)?>)? };
    ($type:ident $(<$generics:tt>)?) => { $type $(<$generics>)? };
}

macro_rules! decode_advance {
    (
        $decoder:ident,
        $key:expr,
        Inline<$type:tt>
    ) => {
        $decoder.decode()?
    };
    (
        $decoder:ident,
        $key:expr,
        Option<$type:tt>
        $(, $default:expr)?
    ) => {
        $decoder.advance_or_none($key)
            $(.map(|v| v.unwrap_or($default)))??
    };
    (
        $decoder:ident,
        $key:expr,
        Vec<$type:tt>
    ) => {
        $decoder.advance_or_default($key)?
    };
    (
        $decoder:ident,
        $key:expr,
        $type:ident $(<$generics:tt>)?
    ) => {
        $decoder.advance_or_err($key)?
    };
    (
        $decoder:ident,
        $key:expr,
        $type:ident $(<$generics:tt>)?,
        $default:expr
    ) => {
        $decoder.advance_or_err($key)
            .unwrap_or($default)
    };
}

macro_rules! ts_response {
    (
        $type:ident $(<$lifetime:lifetime>)? {
            $($field:ident$(($opt_name:expr))?: $field_type:ident $(<$generics:tt $(, $sg:tt)*>)? $(= $default:expr)?),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $type $(<$lifetime>)? {
            $(pub $field: crate::macros::decode_type!($field_type $(<$generics$(, $sg)*>)?)),*
        }

        impl $(<$lifetime>)? crate::parser::Decode for $type $(<$lifetime>)? {
            fn decode(decoder: &mut crate::parser::Decoder) -> Result<Self, crate::error::ParseError> {
                Ok(Self {
                    $(
                        $field: crate::macros::decode_advance!(
                            decoder,
                            crate::macros::decode_key!($field $(, $opt_name)?),
                            $field_type $(<$generics>)?
                            $(, $default)?
                        )
                    ),*
                })
            }
        }
    };
}

macro_rules! ts_enum {
    (
        $type:ident {
            $($name:ident = $value:expr),* $(,)?
        }
    ) => {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $type {
            $($name),*,
            Unknown(String),
        }

        impl crate::parser::DecodeValue for $type {
            fn decode(_key: &str, value: String) -> Result<Self, $crate::error::ParseError> {
                match value.as_ref() {
                    $( stringify!($value) => Ok($type::$name), )*
                    _ => Ok($type::Unknown(value)),
                }
            }
        }

        impl crate::parser::Encode for $type {
            fn encode(&self, buf: &mut String) -> std::fmt::Result {
                match self {
                    $( $type::$name => buf.push_str(stringify!($value)), )*
                    $type::Unknown(value) => buf.push_str(value),
                }

                Ok(())
            }
        }

        impl Default for $type {
            fn default() -> Self {
                $type::Unknown("default".to_string())
            }
        }
    };
}

macro_rules! opt_builder_func {
    (
        $func_name:ident, $name:ident, String
    ) => {
        pub fn $func_name(mut self, $name: impl Into<String>) -> Self {
            self.$name = Some($name.into());
            self
        }
    };
    (
        $func_name:ident, $name:ident, $field_type:ident
    ) => {
        pub fn $func_name(mut self, $name: $field_type) -> Self {
            self.$name = Some($name);
            self
        }
    };
}

macro_rules! opt_builder {
    (
        $type:ident {
            $($name:ident($func_name:ident): $field_type:ident),* $(,)?
        }
    ) => {
        #[derive(Debug, Default, Clone)]
        pub struct $type {
            $(pub $name: Option<$field_type>),*
        }

        impl $type {
            $(
                crate::macros::opt_builder_func!($func_name, $name, $field_type);
            )*
        }
    };
}

macro_rules! flag_builder {
    (
        $type:ident {
            $($name:ident($func_name:ident)),* $(,)?
        }
    ) => {
        #[derive(Debug, Default, Clone)]
        pub struct $type {
            $(pub $name: bool),*
        }

        impl $type {
            pub fn all() -> Self {
                Self {
                    $($name: true),*
                }
            }

            $(
                pub fn $func_name(mut self) -> Self {
                    self.$name = true;
                    self
                }
            )*
        }
    };
}

pub(crate) use property;
pub(crate) use property_type;
pub(crate) use properties;

pub(crate) use permission;
pub(crate) use permission_type;
pub(crate) use permission_parse;
pub(crate) use permissions;

pub(crate) use ts_response;
pub(crate) use decode_key;
pub(crate) use decode_type;
pub(crate) use decode_advance;

pub(crate) use ts_enum;

pub(crate) use opt_builder;
pub(crate) use opt_builder_func;
pub(crate) use flag_builder;
