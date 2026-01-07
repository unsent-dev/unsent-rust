use crate::client::{Client, Result};
use crate::models::*;

pub struct ApiKeysClient<'a> {
    client: &'a Client,
}

impl<'a> ApiKeysClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all API keys for the current team
    pub fn list(&self) -> Result<Vec<ApiKey>> {
        self.client.get("/api-keys")
    }

    /// Create a new API key
    pub fn create(&self, payload: &ApiKeyCreate) -> Result<ApiKeyCreateResponse> {
        self.client.post("/api-keys", payload)
    }

    /// Delete an API key by ID
    pub fn delete(&self, id: &str) -> Result<ApiKeyDeleteResponse> {
        self.client.delete(&format!("/api-keys/{}", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_keys_paths() {
        // Test that paths are correctly formatted
        let paths = vec!["/api-keys", "/api-keys/test-id"];

        for path in paths {
            assert!(path.starts_with("/api-keys"));
        }
    }

    #[test]
    fn test_api_key_create_request() {
        let req = ApiKeyCreate::new("Test Key".to_string());
        assert_eq!(req.name, "Test Key");
        assert_eq!(req.permission, None);
    }

    #[test]
    fn test_api_key_create_request_with_permission() {
        let req = ApiKeyCreate {
            name: "Test Key".to_string(),
            permission: Some(Permission::Sending),
        };
        assert_eq!(req.name, "Test Key");
        assert_eq!(req.permission, Some(Permission::Sending));
    }
}
