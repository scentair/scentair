use std::borrow::Cow;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EmailAddress(String);

#[derive(Debug)]
pub enum EmailAddressError {
    Internal(anyhow::Error),
}

impl EmailAddress {
    pub fn fake() -> Self {
        Self("fake@email.io".to_owned())
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for EmailAddress {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::str::FromStr for EmailAddress {
    type Err = EmailAddressError;

    fn from_str(email_address: &str) -> Result<Self, Self::Err> {
        use std::str::FromStr;

        email_address::EmailAddress::from_str(email_address)
            .map(|email_address| Self(email_address.to_string()))
            .map_err(EmailAddressError::internal)
    }
}

impl EmailAddressError {
    #[inline]
    pub fn internal<E: Into<anyhow::Error>>(error: E) -> Self {
        Self::Internal(error.into())
    }
}
