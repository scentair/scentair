pub struct Argon2;

impl super::PasswordCipher for Argon2 {
    fn hash(plain: &str) -> Result<super::Password, anyhow::Error> {
        use ::argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
        use ::argon2::Argon2;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(plain.as_bytes(), &salt)?.to_string();

        Ok(super::Password {
            hash,
            kind: super::HashKind::Argon2,
        })
    }

    fn verify(plain: &str, hash: &str) -> Result<(), anyhow::Error> {
        use ::argon2::{password_hash::PasswordVerifier, Argon2, PasswordHash};

        let argon2 = Argon2::default();
        let hash = PasswordHash::new(hash)?;
        argon2.verify_password(plain.as_bytes(), &hash)?;

        Ok(())
    }
}
