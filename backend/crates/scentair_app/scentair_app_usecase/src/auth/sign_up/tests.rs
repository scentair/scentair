use super::*;

#[tokio::test]
async fn succeed() {
    struct UserAdapter;
    struct EventAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn exists_by_email_address(
            &self,
            _email_address: &EmailAddress,
            _now: chrono::NaiveDateTime,
        ) -> Result<bool, UseCaseError> {
            Ok(false)
        }

        async fn create(
            &self,
            _password: &Password,
            _name: &UserName,
            _now: chrono::NaiveDateTime,
        ) -> Result<UserId, UseCaseError> {
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
    impl EventRepository for EventAdapter {
        async fn send(
            &self,
            _to: EmailAddress,
            _verification_code: &VerificationToken,
        ) -> Result<(), UseCaseError> {
            Ok(())
        }
    }

    let user = UserAdapter;
    let event = EventAdapter;
    let service = Service::new(user, event);
    let output = service
        .sign_up(
            EmailAddress::fake(),
            Password::fake(),
            UserName::fake(),
            chrono::Utc::now().naive_utc(),
        )
        .await;

    assert_matches!(output, Ok(()));
}

#[tokio::test]
async fn already_taken() {
    struct UserAdapter;
    struct EventAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn exists_by_email_address(
            &self,
            _email_address: &EmailAddress,
            _now: chrono::NaiveDateTime,
        ) -> Result<bool, UseCaseError> {
            Ok(true)
        }
    }

    #[async_trait]
    impl EventRepository for EventAdapter {
        //
    }

    let user = UserAdapter;
    let event = EventAdapter;
    let service = Service::new(user, event);
    let output = service
        .sign_up(
            EmailAddress::fake(),
            Password::fake(),
            UserName::fake(),
            chrono::Utc::now().naive_utc(),
        )
        .await;

    assert_matches!(output, Err(UseCaseError::AlreadyTaken));
}
