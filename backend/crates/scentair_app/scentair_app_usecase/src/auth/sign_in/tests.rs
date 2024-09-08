use super::*;

#[tokio::test]
async fn succeed() {
    struct UserAdapter;
    struct SessionAdapter;
    struct EventAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn find_by_email_address(
            &self,
            email_address: &EmailAddress,
        ) -> Result<UserEntity, UseCaseError> {
            Ok(UserEntity {
                user_id: UserId::new(),
                name: UserName::fake(),
                password: Password::fake(),
            })
        }
    }

    #[async_trait]
    impl SessionRepository for SessionAdapter {
        async fn create(&self, _user_id: UserId) -> Result<SessionToken, UseCaseError> {
            Ok(SessionToken::new())
        }
    }

    #[async_trait]
    impl EventRepository for EventAdapter {
        async fn send(&self, _to: EmailAddress, _name: &UserName) -> Result<(), UseCaseError> {
            Ok(())
        }
    }

    let user = UserAdapter;
    let session = SessionAdapter;
    let event = EventAdapter;
    let service = Service::new(user, session, event);
    let output = service
        .sign_in(EmailAddress::fake(), "password".to_owned())
        .await;

    assert_matches!(output, Ok(_));
}

#[tokio::test]
async fn unknown_email_address() {
    struct UserAdapter;
    struct SessionAdapter;
    struct EventAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn find_by_email_address(
            &self,
            _email_address: &EmailAddress,
        ) -> Result<UserEntity, UseCaseError> {
            Err(UseCaseError::InvalidUser)
        }
    }

    #[async_trait]
    impl SessionRepository for SessionAdapter {
        //
    }

    #[async_trait]
    impl EventRepository for EventAdapter {
        //
    }

    let user = UserAdapter;
    let session = SessionAdapter;
    let event = EventAdapter;
    let service = Service::new(user, session, event);
    let output = service
        .sign_in(EmailAddress::fake(), "password".to_owned())
        .await;

    assert_matches!(output, Err(UseCaseError::InvalidUser));
}
