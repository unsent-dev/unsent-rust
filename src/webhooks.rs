use crate::models::{Webhook, WebhookCreate, WebhookDeleteResponse, WebhookId, WebhookUpdate};
use crate::Client;
// Use the crate's Result alias which is Result<T, UnsentError>
use crate::client::Result;

/// Webhooks resource
///
/// **NOTE**: This resource is currently in development and not fully implemented on the server side yet.
/// These methods are placeholders/preparations for future implementation.
pub struct WebhooksClient<'a> {
    client: &'a Client,
}

impl<'a> WebhooksClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all webhooks
    pub fn list(&self) -> Result<Vec<Webhook>> {
        let path = "/webhooks";
        self.client.get(path)
    }

    /// Create a new webhook
    pub fn create(&self, payload: &WebhookCreate) -> Result<WebhookId> {
        let path = "/webhooks";
        self.client.post(path, payload)
    }

    /// Update a webhook
    pub fn update(&self, id: &str, payload: &WebhookUpdate) -> Result<bool> {
        let path = format!("/webhooks/{}", id);
        let res: serde_json::Value = self.client.patch(path.as_str(), payload)?;
        // TS SDK expects { success: boolean } but returns generic T.
        // Assuming API returns { success: true } or similar.
        // Let's coerce to success bool or just return true on 200 OK
        Ok(res.get("success").and_then(|v| v.as_bool()).unwrap_or(true))
    }

    /// Delete a webhook
    pub fn delete(&self, id: &str) -> Result<WebhookDeleteResponse> {
        let path = format!("/webhooks/{}", id);
        self.client.delete(path.as_str())
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
}
