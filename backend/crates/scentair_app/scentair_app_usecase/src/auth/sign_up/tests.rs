use super::*;

#[tokio::test]
async fn succeed() {
    struct UserAdapter;
    struct EmailAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn exists_by_email_address(&self, _email_address: &EmailAddress) -> bool {
            false
        }

        async fn create(&self, _password: &Password) -> Result<UserId, UseCaseError> {
            Ok(UserId::new())
        }

        async fn add_verification(
            &self,
            _user_id: UserId,
        ) -> Result<VerificationToken, UseCaseError> {
            Ok(VerificationToken::new())
        }
    }

    #[async_trait]
    impl EmailRepository for EmailAdapter {
        async fn send(
            &self,
            _to: EmailAddress,
            _verification_code: &VerificationToken,
        ) -> Result<(), UseCaseError> {
            Ok(())
        }
    }

    let user = UserAdapter;
    let email = EmailAdapter;
    let service = Service::new(user, email);
    let output = service
        .sign_up(EmailAddress::fake(), Password::fake())
        .await;

    assert!(matches!(output, Ok(())));
}

#[tokio::test]
async fn already_taken() {
    struct UserAdapter;
    struct EmailAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn exists_by_email_address(&self, _email_address: &EmailAddress) -> bool {
            true
        }
    }

    #[async_trait]
    impl EmailRepository for EmailAdapter {
        //
    }

    let user = UserAdapter;
    let email = EmailAdapter;
    let service = Service::new(user, email);
    let output = service
        .sign_up(EmailAddress::fake(), Password::fake())
        .await;

    assert!(matches!(output, Err(UseCaseError::AlreadyTaken)));
}
