use std::ops::Not;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserName(String);

#[derive(Debug, Error)]
pub enum UserNameError {
    #[error("invalid username")]
    InvalidPattern,
}

impl UserName {
    #[inline]
    pub fn new(name: &str) -> Result<Self, UserNameError> {
        let name = name.trim();
        validate(name)?;

        Ok(Self(name.to_owned()))
    }

    pub fn fake() -> Self {
        Self("username".to_owned())
    }
}

impl AsRef<str> for UserName {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for UserName {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn validate(name: &str) -> Result<(), UserNameError> {
    match name {
        name if (2..=32).contains(&name.len()).not() => Err(UserNameError::InvalidPattern),
        _ => Ok(()),
    }
}
