#[cfg(test)]
mod tests;

use crypto::{Password, PasswordError};
use domain::{EmailAddress, SessionToken, UserId, UserName};

#[async_trait]
pub trait UseCase {
    async fn sign_in(
        &self,
        email_address: EmailAddress,
        password: String,
    ) -> Result<SessionToken, UseCaseError>;
}

#[async_trait]
pub trait UserRepository: Sync {
    async fn find_by_email_address(
        &self,
        email_address: &EmailAddress,
    ) -> Result<UserEntity, UseCaseError> {
        std::unimplemented!();
    }
}

#[async_trait]
pub trait SessionRepository: Sync {
    async fn create(&self, user_id: UserId) -> Result<SessionToken, UseCaseError> {
        std::unimplemented!();
    }
}

#[async_trait]
pub trait EmailRepository: Sync {
    async fn send(&self, to: EmailAddress, name: &UserName) -> Result<(), UseCaseError> {
        std::unimplemented!();
    }
}

#[derive(Debug)]
pub enum UseCaseError {
    InvalidUser,
    Password(PasswordError),
    Internal(anyhow::Error),
}

pub struct Service<User: UserRepository, Session: SessionRepository, Email: EmailRepository> {
    user: User,
    session: Session,
    email: Email,
}

pub struct UserEntity {
    user_id: UserId,
    name: UserName,
    password: Password,
}

impl<User: UserRepository, Session: SessionRepository, Email: EmailRepository>
    Service<User, Session, Email>
{
    pub const fn new(user: User, session: Session, email: Email) -> Self {
        Self {
            user,
            session,
            email,
        }
    }
}

#[async_trait]
impl<User: UserRepository, Session: SessionRepository, Email: EmailRepository> UseCase
    for Service<User, Session, Email>
{
    async fn sign_in(
        &self,
        email_address: EmailAddress,
        password: String,
    ) -> Result<SessionToken, UseCaseError> {
        let user = self.user.find_by_email_address(&email_address).await?;
        user.password
            .verify(&password)
            .map_err(UseCaseError::Password)?;
        let session_token = self.session.create(user.user_id).await?;
        self.email.send(email_address, &user.name).await?;

        Ok(session_token)
    }
}
