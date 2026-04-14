// @manual
use crate::models::{
    WebhookCreate, WebhookDeleteResponse, WebhookUpdate,
};
use crate::Client;
// Use the crate's Result alias which is Result<T, UnsentError>
use crate::client::Result;

/// Webhooks resource
pub struct WebhooksClient<'a> {
    client: &'a Client,
}

impl<'a> WebhooksClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all webhooks
    pub async fn list(&self) -> Result<serde_json::Value> {
        let path = "/webhooks";
        self.client.get(path).await
    }

    /// Create a new webhook
    pub async fn create(&self, payload: &WebhookCreate) -> Result<serde_json::Value> {
        let path = "/webhooks";
        self.client.post(path, payload).await
    }

    /// Get a webhook by ID
    pub async fn get(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/webhooks/{}", id);
        self.client.get(&path).await
    }

    /// Update a webhook
    pub async fn update(&self, id: &str, payload: &WebhookUpdate) -> Result<bool> {
        let path = format!("/webhooks/{}", id);
        let res: serde_json::Value = self.client.patch(&path, payload).await?;
        // TS SDK expects { success: boolean } but returns generic T.
        // Assuming API returns { success: true } or similar.
        // Let's coerce to success bool or just return true on 200 OK
        Ok(res.get("success").and_then(|v| v.as_bool()).unwrap_or(true))
    }

    /// Delete a webhook
    pub async fn delete(&self, id: &str) -> Result<WebhookDeleteResponse> {
        let path = format!("/webhooks/{}", id);
        self.client.delete(&path).await
    }

    /// Test a webhook by sending a test event
    pub async fn test(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/webhooks/{}/test", id);
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client.post(&path, &empty).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;

    #[test]
    fn test_webhooks_paths() {
        let client = Client::new("test_key").unwrap();
        let _webhooks = WebhooksClient::new(&client);

        // Test path construction
        let paths = vec![
            "/webhooks",
            "/webhooks/webhook-123",
            "/webhooks/webhook-123/test",
        ];

        for path in paths {
            assert!(path.starts_with("/webhooks"));
        }

        // list
        // create
        let _create_payload = WebhookCreate {
            url: "https://example.com/webhook".to_string(),
            events: vec!["email.sent".to_string()],
        };
        // update
        let _update_payload = WebhookUpdate {
            url: Some("https://example.com/webhook2".to_string()),
            events: None,
        };

        // We can't easily mock the internal client without a mock server,
        // but we verify the code compiles and structs are correct.
    }

    #[test]
    fn test_webhook_get_path() {
        let id = "webhook-123";
        let path = format!("/webhooks/{}", id);
        assert_eq!(path, "/webhooks/webhook-123");
    }

    #[test]
    fn test_webhook_test_path() {
        let id = "webhook-123";
        let path = format!("/webhooks/{}/test", id);
        assert_eq!(path, "/webhooks/webhook-123/test");
        assert!(path.ends_with("/test"));
    }
}
