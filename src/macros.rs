macro_rules! property {
    ($value_name:ident, str) => {
        PropertyType::Str($value_name)
    };
    ($value_name:ident, bool) => {
        PropertyType::Bool($value_name)
    };
    ($value_name:ident, i32) => {
        PropertyType::Int($value_name)
    }
}

macro_rules! property_type {
    (str) => {
        &'a str
    };
    (bool) => {
        bool
    };
    (i32) => {
        i32
    }
}

macro_rules! properties {
    ($type:ident {
        $($name:ident: $ty:ident = $value:expr),* $(,)?
    }) => {
        #[allow(dead_code)]
        pub enum $type<'a> {
            $($name($crate::macros::property_type!($ty))),*
        }

        #[allow(dead_code)]
        impl<'a> $type<'a> {
            pub fn contents(self) -> (&'static str, PropertyType<'a>) {
                let name = match self {
                    $( $type::$name { .. } => $value, )*
                };

                let value = match self {
                    $( $type::$name(value) => $crate::macros::property!(value, $ty), )*
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

macro_rules! ts_response {
    ($type:ident {
        $($field:ident$(($str:expr))?: $field_type:ident $(<$generics:ident>)?),* $(,)?
    }) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub struct $type {
            $(pub $field: $field_type $(<$generics>)?),*
        }

        impl $type {
            pub fn from(response: &mut $crate::parser::CommandResponse) -> Result<Self, $crate::error::QueryError> {
                Ok(Self {
                    $($field: response.get::<$field_type$(<$generics>)?>($crate::macros::ts_response_str!($field $(, $str)?))?),*
                })
            }
        }
    }
}

pub(crate) use property;
pub(crate) use property_type;
pub(crate) use properties;
pub(crate) use ts_response;
pub(crate) use ts_response_str;
