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
        now: chrono::NaiveDateTime,
    ) -> Result<(), UseCaseError>;
}

#[async_trait]
pub trait UserRepository: Sync {
    async fn exists_by_email_address(
        &self,
        email_address: &EmailAddress,
        now: chrono::NaiveDateTime,
    ) -> Result<bool, UseCaseError> {
        std::unimplemented!();
    }

    async fn create(
        &self,
        password: &Password,
        name: &UserName,
        now: chrono::NaiveDateTime,
    ) -> Result<UserId, UseCaseError> {
        std::unimplemented!();
    }

    async fn add_verification(&self, user_id: UserId) -> Result<VerificationToken, UseCaseError> {
        std::unimplemented!();
    }
}

#[async_trait]
pub trait EventRepository: Sync {
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

pub struct Service<User: UserRepository, Event: EventRepository> {
    user: User,
    event: Event,
}

impl<User: UserRepository, Event: EventRepository> Service<User, Event> {
    pub const fn new(user: User, event: Event) -> Self {
        Self { user, event }
    }
}

#[async_trait]
impl<User: UserRepository, Event: EventRepository> UseCase for Service<User, Event> {
    async fn sign_up(
        &self,
        email_address: EmailAddress,
        password: Password,
        name: UserName,
        now: chrono::NaiveDateTime,
    ) -> Result<(), UseCaseError> {
        if self
            .user
            .exists_by_email_address(&email_address, now)
            .await?
        {
            return Err(UseCaseError::AlreadyTaken);
        }

        let user_id = self.user.create(&password, &name, now).await?;
        let verification_token = self.user.add_verification(user_id).await?;
        self.event.send(email_address, &verification_token).await?;

        Ok(())
    }
}
