#[cfg(test)]
mod tests;

use crypto::Password;
use domain::{EmailAddress, UserCredentialId, UserId, UserName, VerificationToken};

#[async_trait]
pub trait UseCase {
    async fn sign_up(
        &self,
        email_address: EmailAddress,
        password: Password,
        name: UserName,
    ) -> Result<(), UseCaseError>;
}

#[async_trait]
pub trait UserRepository: Sync {
    async fn exists_by_email_address(&self, email_address: &EmailAddress) -> bool {
        std::unimplemented!();
    }

    async fn create(&self, password: &Password, name: &UserName) -> Result<UserId, UseCaseError> {
        std::unimplemented!();
    }

    async fn add_verification(&self, user_id: UserId) -> Result<VerificationToken, UseCaseError> {
        std::unimplemented!();
    }
}

#[async_trait]
pub trait EmailRepository: Sync {
    async fn send(
        &self,
        to: EmailAddress,
        verification_code: &VerificationToken,
    ) -> Result<(), UseCaseError> {
        std::unimplemented!();
    }
}

#[derive(Debug)]
pub enum UseCaseError {
    AlreadyTaken,
    Internal(anyhow::Error),
}

pub struct Service<User: UserRepository, Email: EmailRepository> {
    user: User,
    email: Email,
}

impl<User: UserRepository, Email: EmailRepository> Service<User, Email> {
    pub const fn new(user: User, email: Email) -> Self {
        Self { user, email }
    }
}

#[async_trait]
impl<User: UserRepository, Email: EmailRepository> UseCase for Service<User, Email> {
    async fn sign_up(
        &self,
        email_address: EmailAddress,
        password: Password,
        name: UserName,
    ) -> Result<(), UseCaseError> {
        if self.user.exists_by_email_address(&email_address).await {
            return Err(UseCaseError::AlreadyTaken);
        }

        let user_id = self.user.create(&password, &name).await?;
        let verification_token = self.user.add_verification(user_id).await?;
        self.email.send(email_address, &verification_token).await?;

        Ok(())
    }
}
