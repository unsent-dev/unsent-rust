use crate::client::{Client, Result};
use crate::models::*;

pub struct SystemClient<'a> {
    client: &'a Client,
}

impl<'a> SystemClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Check if the API is running correctly
    pub fn health(&self) -> Result<HealthResponse> {
        self.client.get("/health")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_system_paths() {
        let path = "/health";
        assert_eq!(path, "/health");
    }
}
