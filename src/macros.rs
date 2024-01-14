macro_rules! property {
    ($value_name:ident, str) => {
        PropertyType::Str($value_name.clone())
    };
    ($value_name:ident, bool) => {
        PropertyType::Bool(*$value_name)
    };
    ($value_name:ident, f32) => {
        PropertyType::Float(*$value_name)
    };
    ($value_name:ident, i32) => {
        PropertyType::Int(*$value_name)
    };
    ($value_name:ident, i64) => {
        PropertyType::Int64(*$value_name)
    };
    ($value_name:ident, $ty:tt) => {
        $value_name.into_property()?
    };
}

macro_rules! property_type {
    (str) => { String };
    (bool) => { bool };
    (f32) => { f32 };
    (i32) => { i32 };
    (i64) => { i64 };
    ($ty:tt) => { $ty };
}

macro_rules! property_parse {
    ($value:expr, bool) => {
        match $value {
            "false" => false,
            "true" => true,
            _ => {
                return Err($crate::error::ParseError::InvalidValue(
                    std::borrow::Cow::from($value.to_string()),
                ))
            }
        }
    };
    ($value:expr, f32) => { $value.parse()? };
    ($value:expr, i32) => { $value.parse()? };
    ($value:expr, i64) => { $value.parse()? };
    ($value:expr, str) => { $value.to_string() };
    ($value:expr, $ty:tt) => { $ty::from($value) };
}

macro_rules! properties {
    ($type:ident {
        $($name:ident: $ty:tt = $value:expr),* $(,)?
    }) => {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq)]
        pub enum $type {
            $($name($crate::macros::property_type!($ty))),*,
            Custom(String, PropertyType),
        }

        #[allow(dead_code)]
        impl $type {
            pub fn parse(id: &str, value: &str, error_on_unknown: bool) -> Result<$type, $crate::error::ParseError> {
                match id {
                    $( $value => Ok($type::$name($crate::macros::property_parse!(value, $ty))), )*
                    _ => if error_on_unknown {
                        Err($crate::error::ParseError::UnknownChannelProperty {
                            id: id.to_string(),
                        })
                    } else {
                        Ok($type::Custom(id.to_owned(), PropertyType::Str(value.to_string())))
                    }
                }
            }

            pub fn contents(&self) -> Result<(std::borrow::Cow<'_, str>, PropertyType), $crate::error::ParseError> {
                let name = match self {
                    $( $type::$name { .. } => std::borrow::Cow::from($value), )*
                    $type::Custom(name, _) => std::borrow::Cow::from(name),
                };

                let value = match self {
                    $( $type::$name(value) => $crate::macros::property!(value, $ty), )*
                    $type::Custom(_, value) => value.clone(),
                };

                Ok((name, value))
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $type {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut state = serializer.serialize_struct(stringify!($type), 2)?;
                let (id, value) = self.contents().map_err(serde::ser::Error::custom)?;

                serde::ser::SerializeStruct::serialize_field(&mut state, "id", id.as_ref())?;

                let value = match value {
                    PropertyType::Str(val) => val,
                    PropertyType::Float(val) => val.to_string(),
                    PropertyType::Int(val) => val.to_string(),
                    PropertyType::Int64(val) => val.to_string(),
                    PropertyType::Bool(val) => val.to_string(),
                };

                serde::ser::SerializeStruct::serialize_field(&mut state, "value", &value)?;
                serde::ser::SerializeStruct::end(state)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<$type, D::Error> {
                #[derive(serde::Deserialize)]
                #[serde(field_identifier, rename_all = "lowercase")]
                enum Field {
                    Id,
                    Value,
                }

                struct PropertyVisitor;

                impl<'de> serde::de::Visitor<'de> for PropertyVisitor {
                    type Value = $type;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(stringify!($type))
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let id = seq.next_element()?.ok_or_else(|| {
                            serde::de::Error::invalid_length(0, &"struct PropertyType with 2 elements")
                        })?;
                        let value = seq.next_element()?.ok_or_else(|| {
                            serde::de::Error::invalid_length(1, &"struct PropertyType with 2 elements")
                        })?;

                        $type::parse(id, value, true).map_err(serde::de::Error::custom)
                    }

                    fn visit_map<V: serde::de::MapAccess<'de>>(
                        self,
                        mut map: V,
                    ) -> Result<Self::Value, V::Error> {
                        let mut id: Option<String> = None;
                        let mut value: Option<String> = None;

                        while let Some(key) = map.next_key()? {
                            match key {
                                Field::Id => {
                                    if id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("id"));
                                    }

                                    id = Some(map.next_value()?);
                                }
                                Field::Value => {
                                    if value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }

                                    value = Some(map.next_value()?);
                                }
                            }
                        }

                        let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                        let value = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;

                        $type::parse(&id, &value, true).map_err(serde::de::Error::custom)
                    }
                }

                const FIELDS: &[&str] = &["id", "value"];
                deserializer.deserialize_struct(stringify!($type), FIELDS, PropertyVisitor)
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
    };
}

macro_rules! permission_parse {
    ($value:expr, bool) => {
        match $value {
            0 => false,
            1 => true,
            _ => {
                return Err($crate::error::ParseError::InvalidValue(
                    std::borrow::Cow::from($value.to_string()),
                ))
            }
        }
    };
    ($value:expr, i32) => {
        $value
    };
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
    ($key:ident) => {
        stringify!($key)
    };
    ($key:ident, $opt_name:expr) => {
        $opt_name
    };
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

macro_rules! ts_string_of {
    ($value:expr, str) => { $value };
    ($value:expr, $ty:tt) => { stringify!($value) };
}

macro_rules! ts_enum_type {
    (str) => { &str };
    ($ty:tt) => { $ty };
}

macro_rules! ts_enum_parse {
    ($value:tt, str) => { $value.as_ref() };
    ($value:tt, $ty:tt) => { $value.parse()? };
}

macro_rules! ts_enum_to_property {
    ($value:tt, str) => { $crate::definitions::PropertyType::Str($value.to_string()) };
    ($value:tt, i32) => { $crate::definitions::PropertyType::Int($value) };
    ($value:tt, bool) => { $crate::definitions::PropertyType::Bool($value) };
}

macro_rules! ts_enum {
    (
        $type:ident<$value_type:tt> {
            $($name:ident = $value:expr),* $(,)?
        }
    ) => {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $type {
            $($name),*,
            Unknown(String),
        }

        impl From<$crate::macros::ts_enum_type!($value_type)> for $type {
            fn from(value: $crate::macros::ts_enum_type!($value_type)) -> Self {
                match value {
                    $( $value => $type::$name, )*
                    _ => $type::Unknown(value.to_string()),
                }
            }
        }

        impl $type {
            pub fn from(value: &str) -> Self {
                match value {
                    $( $crate::macros::ts_string_of!($value, $value_type) => $type::$name, )*
                    _ => $type::Unknown(value.to_string()),
                }
            }

            pub fn descriminator(&self) -> Result<$crate::macros::ts_enum_type!($value_type), $crate::error::ParseError> {
                Ok(match self {
                    $( $type::$name => $value, )*
                    $type::Unknown(value) => $crate::macros::ts_enum_parse!(value, $value_type),
                })
            }

            pub fn into_property(&self) -> Result<$crate::definitions::PropertyType, $crate::error::ParseError> {
                match self {
                    $( $type::$name => Ok($crate::macros::ts_enum_to_property!($value, $value_type)), )*
                    $type::Unknown(value) => Ok($crate::definitions::PropertyType::Str(value.to_string())),
                }
            }
        }

        impl crate::parser::DecodeValue for $type {
            fn decode(_key: &str, value: String) -> Result<Self, $crate::error::ParseError> {
                match value.as_ref() {
                    $( $crate::macros::ts_string_of!($value, $value_type) => Ok($type::$name), )*
                    _ => Ok($type::Unknown(value)),
                }
            }
        }

        impl crate::parser::Encode for $type {
            fn encode(&self, buf: &mut String) -> std::fmt::Result {
                match self {
                    $( $type::$name => buf.push_str($crate::macros::ts_string_of!($value, $value_type)), )*
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

pub(crate) use properties;
pub(crate) use property;
pub(crate) use property_parse;
pub(crate) use property_type;

pub(crate) use permission;
pub(crate) use permission_parse;
pub(crate) use permission_type;
pub(crate) use permissions;

pub(crate) use decode_advance;
pub(crate) use decode_key;
pub(crate) use decode_type;
pub(crate) use ts_response;

pub(crate) use ts_string_of;
pub(crate) use ts_enum_type;
pub(crate) use ts_enum_parse;
pub(crate) use ts_enum_to_property;
pub(crate) use ts_enum;

pub(crate) use flag_builder;
pub(crate) use opt_builder;
pub(crate) use opt_builder_func;
