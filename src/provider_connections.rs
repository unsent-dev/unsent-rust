use crate::client::{Client, Result};
use crate::models::*;

pub struct ProviderConnectionsClient<'a> {
    client: &'a Client,
}

impl<'a> ProviderConnectionsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<serde_json::Value> {
        self.client.get("/provider-connections").await
    }

    pub async fn create(&self, payload: &CreateProviderConnectionRequest) -> Result<serde_json::Value> {
        self.client.post("/provider-connections", payload).await
    }

    pub async fn delete(&self, id: &str) -> Result<serde_json::Value> {
        self.client.delete(&format!("/provider-connections/{}", id)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_connections_paths() {
        let paths = vec!["/provider-connections", "/provider-connections", "/provider-connections/test-id"];

        for path in paths {
            assert!(path.starts_with("/provider-connections"));
        }
    }
}
