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
        $($name:ident: $ty:ident = $value:expr),*
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

macro_rules! ts_response_field {
    ($response:ident, $field:ident, String) => {
        let $field = $response.get(stringify!($field))?;
    };
    ($response:ident, $field:ident, i32) => {
        let $field = $response.get_i32(stringify!($field))?;
    };
    ($response:ident, $field:ident, bool) => {
        let $field = $response.get_bool(stringify!($field))?;
    };
    ($response:ident, $field:ident, Vec<String>) => {
        let $field = $response.get_list(stringify!($field))?;
    };
    ($response:ident, $field:ident, Vec<i32>) => {
        let $field = $response.get_i32_list(stringify!($field))?;
    };
    ($response:ident, $field:ident, $ty:ty) => {
        let $field = $response.get(stringify!($field))?;
        let $field = <$ty>::from_str(&$field)?;
    };
}

macro_rules! ts_response {
    ($type:ident {
        $($field:ident: $field_type:ident $(<$generics:ident>)?),*
    }) => {
        #[allow(dead_code, non_snake_case)]
        #[derive(Debug)]
        pub struct $type {
            pub $($field: $field_type $(<$generics>)?),*
        }

        #[allow(non_snake_case)]
        impl $type {
            pub fn from(response: &mut $crate::parser::CommandResponse) -> Result<Self, $crate::error::QueryError> {
                $(
                    $crate::macros::ts_response_field!(response, $field, $field_type $(<$generics>)?);
                )*

                Ok(Self {
                    $($field),*
                })
            }
        }
    }
}

pub(crate) use property;
pub(crate) use property_type;
pub(crate) use properties;
pub(crate) use ts_response;
pub(crate) use ts_response_field;