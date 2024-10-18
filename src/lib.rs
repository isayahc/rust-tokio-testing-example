use tokio;

pub struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    pub fn new(id: u32, name: String, email: String) -> Self {
        User { id, name, email }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub async fn fetch_data(&self) -> String {
        // Simulate an asynchronous operation
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        format!("Data for user {}: {} ({})", self.id, self.name, self.email)
    }
}
