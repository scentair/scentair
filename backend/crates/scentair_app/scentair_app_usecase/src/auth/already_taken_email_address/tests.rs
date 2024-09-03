use super::*;

#[tokio::test]
async fn succeed() {
    struct UserAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn exists_by_email_address(&self, _email_address: &EmailAddress) -> bool {
            false
        }
    }

    let user = UserAdapter;
    let service = Service::new(user);
    let output = service
        .already_taken_email_address(EmailAddress::fake())
        .await;

    assert!(output.is_ok());
}

#[tokio::test]
async fn already_taken() {
    struct UserAdapter;

    #[async_trait]
    impl UserRepository for UserAdapter {
        async fn exists_by_email_address(&self, _email_address: &EmailAddress) -> bool {
            true
        }
    }

    let user = UserAdapter;
    let service = Service::new(user);
    let output = service
        .already_taken_email_address(EmailAddress::fake())
        .await;

    assert_matches!(output, Err(UseCaseError::AlreadyTaken));
}
