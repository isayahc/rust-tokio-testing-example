use tokio_example::User;

#[tokio::test]
async fn test_user_creation_and_getters() {
    let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    
    assert_eq!(user.id(), 1);
    assert_eq!(user.name(), "Alice");
    assert_eq!(user.email(), "alice@example.com");
}

#[tokio::test]
async fn test_user_fetch_data() {
    let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    
    let result = user.fetch_data().await;
    assert_eq!(result, "Data for user 1: Alice (alice@example.com)");
}