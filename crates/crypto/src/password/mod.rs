mod argon2;

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
    #[error("Unknown hash kind")]
    UnknownHashKind,
    #[error("{0}")]
    Internal(#[from] anyhow::Error),
}

impl Password {
    pub fn from_plain(plain: &str) -> Result<Self, PasswordError> {
        let password = argon2::Argon2::hash(plain)?;

        Ok(password)
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
