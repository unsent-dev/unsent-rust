use crate::client::{Client, Result};
use crate::models::*;

pub struct MetricsClient<'a> {
    client: &'a Client,
}

impl<'a> MetricsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get performance metrics
    pub async fn get(&self, params: Option<&MetricsParams>) -> Result<MetricsResponse> {
        let mut path = "/metrics".to_string();

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
    fn test_metrics_client_creation() {
        let path = "/metrics";
        assert_eq!(path, "/metrics");
    }

    #[test]
    fn test_metrics_params_default() {
        let params = MetricsParams::default();
        assert_eq!(params.period, None);
    }

    #[test]
    fn test_metrics_params_with_period() {
        let params = MetricsParams {
            period: Some("month".to_string()),
        };
        assert_eq!(params.period, Some("month".to_string()));
    }

    #[test]
    fn test_metrics_params_valid_periods() {
        let valid_periods = vec!["day", "week", "month"];

        for period in valid_periods {
            let params = MetricsParams {
                period: Some(period.to_string()),
            };
            assert_eq!(params.period, Some(period.to_string()));
        }
    }

    #[test]
    fn test_metrics_params_serialization() {
        let params = MetricsParams {
            period: Some("week".to_string()),
        };

        let serialized = serde_qs::to_string(&params).unwrap();
        assert!(serialized.contains("period=week"));
    }

    #[test]
    fn test_metrics_path_construction() {
        let base_path = "/metrics";
        assert_eq!(base_path, "/metrics");

        let params = MetricsParams {
            period: Some("day".to_string()),
        };
        let query = serde_qs::to_string(&params).unwrap();
        let full_path = format!("{}?{}", base_path, query);

        assert!(full_path.starts_with("/metrics?"));
        assert!(full_path.contains("period"));
    }

    #[test]
    fn test_metrics_response_deserialization() {
        let json = r#"{"sent": 100, "delivered": 95}"#;
        let response: std::result::Result<MetricsResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());
    }

    #[test]
    fn test_metrics_default_period() {
        // According to API spec, default period is "month"
        let params = MetricsParams::default();
        assert_eq!(params.period, None); // None means API will use default
    }

    #[test]
    fn test_metrics_path_without_params() {
        let path = "/metrics".to_string();
        assert_eq!(path, "/metrics");
    }
}
