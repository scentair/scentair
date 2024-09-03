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
pub trait EmailRepository: Sync {
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
    async fn accept_sign_up_token(&self, token: VerificationToken) -> Result<(), UseCaseError> {
        let user_verification_id = self.user.find_by_verification_token(&token).await?;
        self.user.verify_sign_up_token(user_verification_id).await?;

        let user = self
            .user
            .find_by_user_verification_id(user_verification_id)
            .await?;
        self.email.send(&user.email_address, &user.name).await?;

        Ok(())
    }
}
