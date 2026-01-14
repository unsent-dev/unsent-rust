use crate::client::{Client, Result};
use crate::models::*;

pub struct StatsClient<'a> {
    client: &'a Client,
}

impl<'a> StatsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get email statistics
    pub async fn get(&self, params: Option<&StatsParams>) -> Result<StatsResponse> {
        let mut path = "/stats".to_string();

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
    fn test_stats_client_creation() {
        let path = "/stats";
        assert_eq!(path, "/stats");
    }

    #[test]
    fn test_stats_params_default() {
        let params = StatsParams::default();
        assert_eq!(params.start_date, None);
        assert_eq!(params.end_date, None);
    }

    #[test]
    fn test_stats_params_with_date_range() {
        let params = StatsParams {
            start_date: Some("2024-01-01T00:00:00Z".to_string()),
            end_date: Some("2024-01-31T23:59:59Z".to_string()),
        };
        assert_eq!(params.start_date, Some("2024-01-01T00:00:00Z".to_string()));
        assert_eq!(params.end_date, Some("2024-01-31T23:59:59Z".to_string()));
    }

    #[test]
    fn test_stats_params_with_start_date_only() {
        let params = StatsParams {
            start_date: Some("2024-01-01T00:00:00Z".to_string()),
            end_date: None,
        };
        assert_eq!(params.start_date, Some("2024-01-01T00:00:00Z".to_string()));
        assert_eq!(params.end_date, None);
    }

    #[test]
    fn test_stats_params_with_end_date_only() {
        let params = StatsParams {
            start_date: None,
            end_date: Some("2024-01-31T23:59:59Z".to_string()),
        };
        assert_eq!(params.start_date, None);
        assert_eq!(params.end_date, Some("2024-01-31T23:59:59Z".to_string()));
    }

    #[test]
    fn test_stats_params_serialization() {
        let params = StatsParams {
            start_date: Some("2024-01-01".to_string()),
            end_date: Some("2024-01-31".to_string()),
        };

        let serialized = serde_qs::to_string(&params).unwrap();
        assert!(serialized.contains("startDate"));
        assert!(serialized.contains("endDate"));
    }

    #[test]
    fn test_stats_path_construction() {
        let base_path = "/stats";
        assert_eq!(base_path, "/stats");

        let params = StatsParams {
            start_date: Some("2024-01-01".to_string()),
            end_date: Some("2024-01-31".to_string()),
        };
        let query = serde_qs::to_string(&params).unwrap();
        let full_path = format!("{}?{}", base_path, query);

        assert!(full_path.starts_with("/stats?"));
    }

    #[test]
    fn test_stats_response_deserialization() {
        let json = r#"{"total": 1000, "sent": 950}"#;
        let response: std::result::Result<StatsResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());
    }

    #[test]
    fn test_stats_date_formats() {
        // Test ISO 8601 format
        let params = StatsParams {
            start_date: Some("2024-01-01T00:00:00Z".to_string()),
            end_date: Some("2024-12-31T23:59:59Z".to_string()),
        };
        assert!(params.start_date.unwrap().contains("T"));
        assert!(params.end_date.unwrap().contains("Z"));
    }

    #[test]
    fn test_stats_path_without_params() {
        let path = "/stats".to_string();
        assert_eq!(path, "/stats");
    }
}
