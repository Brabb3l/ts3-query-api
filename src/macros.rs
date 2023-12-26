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

macro_rules! permission {
    ($value_name:ident, bool) => {
        PermissionValue::Bool(*$value_name)
    };
    ($value_name:ident, i32) => {
        PermissionValue::Int(*$value_name)
    };
}

macro_rules! permission_value {
    (bool) => {
        bool
    };
    (i32) => {
        i32
    }
}

macro_rules! permissions {
    ($type:ident {
        $($name:ident: $ty:ident),* $(,)?
    }) => {
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub enum $type<'a> {
            $($name($crate::macros::permission_value!($ty))),*,
            Custom(&'a str, PermissionValue),
        }

        #[allow(dead_code)]
        impl<'a> $type<'a> {
            pub fn contents(&'a self) -> (&'a str, PermissionValue) {
                let name = match self {
                    $( $type::$name { .. } => stringify!($name), )*
                    $type::Custom(name, _) => name,
                };

                let value = match self {
                    $( $type::$name(value) => $crate::macros::permission!(value, $ty), )*
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
        #[derive(Debug, Default)]
        pub struct $type {
            $(pub $name: Option<$field_type>),*
        }

        impl $type {
            pub fn new() -> Self {
                Self::default()
            }

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
        #[derive(Debug, Default)]
        pub struct $type {
            $(pub $name: bool),*
        }

        impl $type {
            pub fn new() -> Self {
                Self::default()
            }

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
pub(crate) use permission_value;
pub(crate) use permissions;

pub(crate) use ts_response;
pub(crate) use ts_response_str;
pub(crate) use ts_response_getter;

pub(crate) use ts_enum;

pub(crate) use opt_builder;
pub(crate) use opt_builder_func;
pub(crate) use flag_builder;
