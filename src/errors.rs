use core::{error::Error, fmt};

// Inherited from h3o
// https://github.com/HydroniumLabs/h3o/blob/ad2bebf52eab218d66b0bf213b14a2802bf616f7/src/error/invalid_value.rs

// Macro to declare type-specific InvalidValue error type.
macro_rules! invalid_value_error {
    ($name:literal, $error:ident, $value_type:ty) => {
        #[doc = concat!("Invalid ", $name, ".")]
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[allow(
            clippy::allow_attributes,
            clippy::derive_partial_eq_without_eq,
            reason = "value type may not be `Eq` (e.g. f64)"
        )]
        pub struct $error {
            /// The invalid value.
            pub value: $value_type,
            /// The reason why it's invalid.
            pub reason: &'static str,
        }

        impl $error {
            pub(crate) const fn new(value: $value_type, reason: &'static str) -> Self {
                Self { value, reason }
            }
        }

        impl fmt::Display for $error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "invalid {} (got {:?}): {}",
                    $name, self.value, self.reason
                )
            }
        }

        impl Error for $error {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                None
            }
        }
    };
}

invalid_value_error!("direction", InvalidDirection, u8);
