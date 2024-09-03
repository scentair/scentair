use super::*;

#[tokio::test]
async fn succeed() {
    struct UserAdapter;
    struct EmailAdapter;

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

        async fn find_by_user_verification_id(
            &self,
            _user_verification_id: UserVerificationId,
        ) -> Result<UserEntity, UseCaseError> {
            Ok(UserEntity {
                name: UserName::fake(),
                email_address: EmailAddress::fake(),
            })
        }
    }

    #[async_trait]
    impl EmailRepository for EmailAdapter {
        async fn send(&self, _to: &EmailAddress, _name: &UserName) -> Result<(), UseCaseError> {
            Ok(())
        }
    }

    let user = UserAdapter;
    let email = EmailAdapter;
    let service = Service::new(user, email);
    let output = service.accept_sign_up_token(VerificationToken::new()).await;

    assert!(output.is_ok());
}

#[tokio::test]
async fn invalid_token() {
    struct UserAdapter;
    struct EmailAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn find_by_verification_token(
            &self,
            _token: &VerificationToken,
        ) -> Result<UserVerificationId, UseCaseError> {
            Err(UseCaseError::InvalidToken)
        }
    }

    #[async_trait]
    impl EmailRepository for EmailAdapter {
        //
    }

    let user = UserAdapter;
    let email = EmailAdapter;
    let service = Service::new(user, email);
    let output = service.accept_sign_up_token(VerificationToken::new()).await;

    assert_matches!(output, Err(UseCaseError::InvalidToken));
}
