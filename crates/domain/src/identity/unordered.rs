#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UnorderedIdentity(pub(crate) u128);

impl UnorderedIdentity {
    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4();

        Self(uuid.as_u128())
    }

    #[inline]
    pub const fn as_u128(self) -> u128 {
        self.0
    }
}

#[derive(Debug)]
pub enum UnorderedIdentityError {
    InvalidPrefix,
    InvalidValue,
}

#[macro_export]
macro_rules! new_unordered_identity {
    ($name:ident, $prefix:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name(crate::UnorderedIdentity);

        impl $name {
            #[inline]
            pub fn new() -> Self {
                Self(crate::UnorderedIdentity::new())
            }

            #[inline]
            pub const fn from_u128(value: u128) -> Self {
                Self(crate::UnorderedIdentity(value))
            }

            #[inline]
            pub const fn as_u128(self) -> u128 {
                self.0.as_u128()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{:x}", $prefix, self.as_u128())
            }
        }

        impl std::str::FromStr for $name {
            type Err = UnorderedIdentityError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value.strip_prefix($prefix) {
                    Some(value) => match u128::from_str_radix(value, 16) {
                        Ok(value) => Ok(Self::from_u128(value)),
                        Err(_) => Err(UnorderedIdentityError::InvalidValue),
                    },
                    None => Err(UnorderedIdentityError::InvalidPrefix),
                }
            }
        }
    };
}
