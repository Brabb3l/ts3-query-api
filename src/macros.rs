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

macro_rules! ts_response_str {
    ($field:ident) => {
        stringify!($field)
    };
    ($field:ident, $str:expr) => {
        $str
    }
}

macro_rules! ts_response_getter {
    ($response:expr, $field_value:expr) => {
        $response.get($field_value)
    };
    ($response:expr, $field_value:expr, $default:expr) => {
        $response.get_or($field_value, || $default)
    };
}

macro_rules! ts_response {
    ($type:ident {
        $($field:ident$(($str:expr))?: $field_type:ident $(<$generics:ident>)? $(= $default:expr)?),* $(,)?
    }) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub struct $type {
            $(pub $field: $field_type $(<$generics>)?),*
        }

        impl $type {
            pub fn from(response: &mut $crate::parser::CommandResponse) -> Result<Self, $crate::error::QueryError> {
                Ok(Self {
                    $($field: $crate::macros::ts_response_getter!(
                        response,
                        $crate::macros::ts_response_str!($field $(, $str)?)
                        $(, $default)?
                    )?),*
                })
            }
        }
    }
}

macro_rules! ts_enum {
    (
        $type:ident {
            $($name:ident = $value:expr),* $(,)?
        }
    ) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum $type {
            $($name),*,
            Unknown(String),
        }

        impl crate::parser::Decode for $type {
            fn decode(_key: &str, value: String) -> Result<Self, $crate::error::QueryError> {
                match value.as_str() {
                    $( stringify!($value) => Ok($type::$name), )*
                    _ => Ok($type::Unknown(value)),
                }
            }
        }

        impl Default for $type {
            fn default() -> Self {
                $type::Unknown("default".to_string())
            }
        }
    };
}

pub(crate) use property;
pub(crate) use property_type;
pub(crate) use properties;
pub(crate) use ts_response;
pub(crate) use ts_response_str;
pub(crate) use ts_response_getter;
pub(crate) use ts_enum;
