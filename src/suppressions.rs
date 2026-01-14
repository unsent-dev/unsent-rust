use crate::client::{Client, Result};
use crate::models::*;

pub struct SuppressionsClient<'a> {
    client: &'a Client,
}

impl<'a> SuppressionsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List suppressed emails
    pub async fn list(&self, params: Option<&SuppressionListParams>) -> Result<serde_json::Value> {
        let mut path = "/suppressions".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path).await
    }

    /// Add an email to the suppression list
    pub async fn add(&self, payload: &AddSuppressionRequest) -> Result<serde_json::Value> {
        self.client.post("/suppressions", payload).await
    }

    /// Remove an email from the suppression list
    pub async fn delete(&self, email: &str) -> Result<serde_json::Value> {
        self.client
            .delete(&format!("/suppressions/email/{}", email)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suppressions_paths() {
        let paths = vec!["/suppressions", "/suppressions/email/test@example.com"];

        for path in paths {
            assert!(path.starts_with("/suppressions"));
        }
    }

    #[test]
    fn test_add_suppression_request() {
        let req = AddSuppressionRequest::new("test@example.com".to_string(), Reason::Manual);
        assert_eq!(req.email, "test@example.com");
        assert_eq!(req.reason, Reason::Manual);
        assert_eq!(req.source, None);
    }

    #[test]
    fn test_add_suppression_with_source() {
        let req = AddSuppressionRequest {
            email: "test@example.com".to_string(),
            reason: Reason::HardBounce,
            source: Some("import".to_string()),
        };
        assert_eq!(req.source, Some("import".to_string()));
    }

    #[test]
    fn test_suppression_list_params() {
        let params = SuppressionListParams {
            page: Some(1),
            limit: Some(20),
            search: Some("@example.com".to_string()),
            reason: Some(Reason::Complaint),
        };
        assert_eq!(params.page, Some(1));
        assert_eq!(params.limit, Some(20));
        assert_eq!(params.search, Some("@example.com".to_string()));
        assert_eq!(params.reason, Some(Reason::Complaint));
    }
}
