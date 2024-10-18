use tokio_example::User;

#[tokio::main]
async fn main() {
    let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    let result = user.fetch_data().await;
    println!("Result: {}", result);
}