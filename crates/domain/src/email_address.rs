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
}

impl std::str::FromStr for EmailAddress {
    type Err = EmailAddressError;

    fn from_str(email_address: &str) -> Result<Self, Self::Err> {
        use std::str::FromStr;

        match email_address::EmailAddress::from_str(email_address) {
            Ok(email_address) => Ok(Self(email_address.to_string())),
            Err(error) => Err(EmailAddressError::Internal(error.into())),
        }
    }
}
