mod argon2;

use crate::password;
use std::{borrow::Cow, ops::Not};

trait PasswordCipher {
    fn hash(plain: &str) -> Result<Password, anyhow::Error>;

    fn verify(plain: &str, hash: &str) -> Result<(), anyhow::Error>;
}

pub struct Password {
    hash: String,
    kind: HashKind,
}

#[derive(Debug)]
pub enum HashKind {
    Argon2,
}

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password does not meet the pattern requirements")]
    InvalidPasswordPattern,
    #[error("Unknown hash kind")]
    UnknownHashKind,
    #[error("{0}")]
    Internal(#[from] anyhow::Error),
}

impl Password {
    pub fn fake() -> Self {
        Self {
            hash: "fake".to_owned(),
            kind: HashKind::Argon2,
        }
    }
}

impl Password {
    pub fn from_plain(plain: &str) -> Result<Self, PasswordError> {
        validate(plain)?;

        Ok(argon2::Argon2::hash(plain)?)
    }

    pub fn from_encoded(encoded: &str) -> Result<Self, PasswordError> {
        let (source, kind) = match encoded.split_once(':') {
            Some(("Argon2", source)) => (source.to_string(), HashKind::Argon2),
            _ => return Err(PasswordError::UnknownHashKind),
        };

        Ok(Self { hash: source, kind })
    }

    pub fn verify(&self, plain: &str, hash: &str) -> Result<(), PasswordError> {
        match self.kind {
            HashKind::Argon2 => argon2::Argon2::verify(plain, &self.hash)?,
        }

        Ok(())
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.kind, self.hash)
    }
}

fn validate(password: &str) -> Result<(), PasswordError> {
    macro_rules! invalid {
        () => {
            return Err(PasswordError::InvalidPasswordPattern)
        };
    }

    match password {
        password if (8..=64).contains(&password.len()).not() => invalid!(),
        password if password.chars().any(char::is_numeric) => invalid!(),
        password if password.chars().any(char::is_uppercase) => invalid!(),
        password if password.chars().any(char::is_lowercase) => invalid!(),
        password if password.chars().any(|c| c.is_alphanumeric().not()) => invalid!(),
        _ => {}
    }

    let chars: Vec<_> = password.chars().collect();
    if chars.windows(3).any(|w| w[0] == w[1] && w[1] == w[2]) {
        invalid!();
    }

    Ok(())
}
