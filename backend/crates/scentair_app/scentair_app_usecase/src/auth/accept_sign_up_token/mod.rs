#[cfg(test)]
mod tests;

use domain::{UserVerificationId, VerificationToken};

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
}

#[derive(Debug)]
pub enum UseCaseError {
    InvalidToken,
    Internal(anyhow::Error),
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
    async fn accept_sign_up_token(&self, token: VerificationToken) -> Result<(), UseCaseError> {
        let user_verification_id = self.user.find_by_verification_token(&token).await?;
        self.user.verify_sign_up_token(user_verification_id).await?;

        Ok(())
    }
}
