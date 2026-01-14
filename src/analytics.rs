use crate::client::{Client, Result};
use crate::models::*;

pub struct AnalyticsClient<'a> {
    client: &'a Client,
}

impl<'a> AnalyticsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get email analytics
    pub async fn get(&self) -> Result<GetAnalytics200Response> {
        self.client.get("/analytics").await
    }

    /// Get analytics time series data
    pub async fn time_series(&self, params: Option<&TimeSeriesParams>) -> Result<TimeSeriesResponse> {
        let mut path = "/analytics/time-series".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path).await
    }

    /// Get sender reputation metrics
    pub async fn reputation(&self, params: Option<&ReputationParams>) -> Result<ReputationResponse> {
        let mut path = "/analytics/reputation".to_string();

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
    fn test_analytics_paths() {
        let paths = vec![
            "/analytics",
            "/analytics/time-series",
            "/analytics/reputation",
        ];

        for path in paths {
            assert!(path.starts_with("/analytics"));
        }
    }

    #[test]
    fn test_time_series_params() {
        let params = TimeSeriesParams {
            days: Some("30".to_string()),
            domain: Some("example.com".to_string()),
        };
        assert_eq!(params.days, Some("30".to_string()));
        assert_eq!(params.domain, Some("example.com".to_string()));
    }

    #[test]
    fn test_reputation_params() {
        let params = ReputationParams {
            domain: Some("example.com".to_string()),
        };
        assert_eq!(params.domain, Some("example.com".to_string()));
    }
}
