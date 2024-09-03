#[cfg(test)]
mod tests;

use domain::EmailAddress;

#[async_trait]
pub trait UseCase {
    async fn already_taken_email_address(
        &self,
        email_address: EmailAddress,
        now: chrono::NaiveDateTime,
    ) -> Result<(), UseCaseError>;
}

#[async_trait]
pub trait UserRepository: Sync {
    async fn exists_by_email_address(
        &self,
        email_address: &EmailAddress,
        now: chrono::NaiveDateTime,
    ) -> Result<bool, UseCaseError>;
}

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("The email address is already taken")]
    AlreadyTaken,
    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

pub struct Service<User: UserRepository> {
    user: User,
}

impl<User: UserRepository> Service<User> {
    pub const fn new(user: User) -> Self {
        Self { user }
    }
}

#[async_trait]
impl<User: UserRepository> UseCase for Service<User> {
    async fn already_taken_email_address(
        &self,
        email_address: EmailAddress,
        now: chrono::NaiveDateTime,
    ) -> Result<(), UseCaseError> {
        if self
            .user
            .exists_by_email_address(&email_address, now)
            .await?
        {
            return Err(UseCaseError::AlreadyTaken);
        }

        Ok(())
    }
}
