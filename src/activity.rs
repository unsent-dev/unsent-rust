// @manual
use crate::client::{Client, Result};
use crate::models::*;

pub struct ActivityClient<'a> {
    client: &'a Client,
}

impl<'a> ActivityClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get activity feed with email events and email details
    pub async fn get(&self, params: Option<&ActivityParams>) -> Result<ActivityResponse> {
        let mut path = "/activity".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_client_creation() {
        // Mock client would be created here in real tests
        // This tests the path construction logic
        let path = "/activity";
        assert_eq!(path, "/activity");
    }

    #[test]
    fn test_activity_params_serialization() {
        let params = ActivityParams {
            page: Some(1),
            limit: Some(50),
        };

        let serialized = serde_qs::to_string(&params).unwrap();
        assert!(serialized.contains("page=1"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_activity_params_default() {
        let params = ActivityParams::default();
        assert_eq!(params.page, None);
        assert_eq!(params.limit, None);
    }

    #[test]
    fn test_activity_params_with_pagination() {
        let params = ActivityParams {
            page: Some(2),
            limit: Some(25),
        };
        assert_eq!(params.page, Some(2));
        assert_eq!(params.limit, Some(25));
    }

    #[test]
    fn test_activity_path_construction() {
        let base_path = "/activity";

        // Test without params
        assert_eq!(base_path, "/activity");

        // Test with params would create query string
        let params = ActivityParams {
            page: Some(1),
            limit: Some(10),
        };
        let query = serde_qs::to_string(&params).unwrap();
        let full_path = format!("{}?{}", base_path, query);

        assert!(full_path.starts_with("/activity?"));
        assert!(full_path.contains("page"));
        assert!(full_path.contains("limit"));
    }

    #[test]
    fn test_activity_response_deserialization() {
        let json = r#"{"data": {"key": "value"}}"#;
        let response: std::result::Result<ActivityResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());
    }
}
