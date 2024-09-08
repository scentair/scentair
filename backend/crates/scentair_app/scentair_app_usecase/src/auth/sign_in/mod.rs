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
pub trait EventRepository: Sync {
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

pub struct Service<User: UserRepository, Session: SessionRepository, Event: EventRepository> {
    user: User,
    session: Session,
    event: Event,
}

pub struct UserEntity {
    user_id: UserId,
    name: UserName,
    password: Password,
}

impl<User: UserRepository, Session: SessionRepository, Event: EventRepository>
    Service<User, Session, Event>
{
    pub const fn new(user: User, session: Session, event: Event) -> Self {
        Self {
            user,
            session,
            event,
        }
    }
}

#[async_trait]
impl<User: UserRepository, Session: SessionRepository, Event: EventRepository> UseCase
    for Service<User, Session, Event>
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
        self.event.send(email_address, &user.name).await?;

        Ok(session_token)
    }
}
