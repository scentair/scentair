#[cfg(test)]
mod tests;

use domain::{EmailAddress, UserName, UserVerificationId, VerificationToken};

#[async_trait]
pub trait UseCase {
    async fn accept_sign_up_token(&self, token: VerificationToken) -> Result<(), UseCaseError>;
}

#[async_trait]
pub trait UserRepository: Sync {
    async fn find_by_verification_token(
        &self,
        token: &VerificationToken,
    ) -> Result<UserVerificationId, UseCaseError> {
        std::unimplemented!()
    }

    async fn verify_sign_up_token(
        &self,
        user_verification_id: UserVerificationId,
    ) -> Result<(), UseCaseError> {
        std::unimplemented!()
    }

    async fn find_by_user_verification_id(
        &self,
        user_verification_id: UserVerificationId,
    ) -> Result<UserEntity, UseCaseError> {
        std::unimplemented!()
    }
}

#[async_trait]
pub trait EventRepository: Sync {
    async fn send(&self, to: &EmailAddress, name: &UserName) -> Result<(), UseCaseError> {
        std::unimplemented!()
    }
}

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

pub struct UserEntity {
    pub name: UserName,
    pub email_address: EmailAddress,
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
    async fn accept_sign_up_token(&self, token: VerificationToken) -> Result<(), UseCaseError> {
        let user_verification_id = self.user.find_by_verification_token(&token).await?;
        self.user.verify_sign_up_token(user_verification_id).await?;

        let user = self
            .user
            .find_by_user_verification_id(user_verification_id)
            .await?;
        self.event.send(&user.email_address, &user.name).await?;

        Ok(())
    }
}
