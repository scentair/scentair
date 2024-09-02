use super::*;

#[tokio::test]
async fn succeed() {
    struct UserAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn find_by_verification_token(
            &self,
            _token: &VerificationToken,
        ) -> Result<UserVerificationId, UseCaseError> {
            Ok(UserVerificationId::new())
        }

        async fn verify_sign_up_token(
            &self,
            _user_verification_id: UserVerificationId,
        ) -> Result<(), UseCaseError> {
            Ok(())
        }
    }

    let user = UserAdapter;
    let service = Service::new(user);
    let output = service.accept_sign_up_token(VerificationToken::new()).await;

    assert!(output.is_ok());
}

#[tokio::test]
async fn invalid_token() {
    struct UserAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn find_by_verification_token(
            &self,
            _token: &VerificationToken,
        ) -> Result<UserVerificationId, UseCaseError> {
            Err(UseCaseError::InvalidToken)
        }
    }

    let user = UserAdapter;
    let service = Service::new(user);
    let output = service.accept_sign_up_token(VerificationToken::new()).await;

    assert!(matches!(output, Err(UseCaseError::InvalidToken)));
}
