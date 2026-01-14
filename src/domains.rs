use crate::client::{Client, Result};
use crate::models::*;

pub struct DomainsClient<'a> {
    client: &'a Client,
}

impl<'a> DomainsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<Domain>> {
        self.client.get("/domains").await
    }

    pub async fn create(&self, payload: &DomainCreate) -> Result<DomainCreateResponse> {
        self.client.post("/domains", payload).await
    }

    pub async fn verify(&self, domain_id: &str) -> Result<DomainVerifyResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client
            .put(&format!("/domains/{}/verify", domain_id), &empty).await
    }

    pub async fn get(&self, domain_id: &str) -> Result<Domain> {
        self.client.get(&format!("/domains/{}", domain_id)).await
    }

    pub async fn delete(&self, domain_id: &str) -> Result<DomainDeleteResponse> {
        self.client.delete(&format!("/domains/{}", domain_id)).await
    }

    /// Get analytics for a specific domain
    pub async fn get_analytics(
        &self,
        domain_id: &str,
        params: Option<&DomainAnalyticsParams>,
    ) -> Result<Vec<DomainAnalytics>> {
        let mut path = format!("/domains/{}/analytics", domain_id);

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path).await
    }

    /// Get stats for a specific domain
    pub async fn get_stats(
        &self,
        domain_id: &str,
        params: Option<&DomainStatsParams>,
    ) -> Result<DomainStats> {
        let mut path = format!("/domains/{}/stats", domain_id);

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
    fn test_domains_paths() {
        let paths = vec!["/domains", "/domains/test-id", "/domains/test-id/verify"];

        for path in paths {
            assert!(path.starts_with("/domains"));
        }
    }

    #[test]
    fn test_domain_create() {
        let req = DomainCreate::new("example.com".to_string(), "us-east-1".to_string());
        assert_eq!(req.name, "example.com");
        assert_eq!(req.region, "us-east-1");
    }

    #[test]
    fn test_domain_analytics_path() {
        let domain_id = "domain-123";
        let path = format!("/domains/{}/analytics", domain_id);
        assert_eq!(path, "/domains/domain-123/analytics");
        assert!(path.contains("analytics"));
    }

    #[test]
    fn test_domain_stats_path() {
        let domain_id = "domain-123";
        let path = format!("/domains/{}/stats", domain_id);
        assert_eq!(path, "/domains/domain-123/stats");
        assert!(path.contains("stats"));
    }

    #[test]
    fn test_domain_analytics_params() {
        let params = DomainAnalyticsParams {
            period: Some("week".to_string()),
        };
        assert_eq!(params.period, Some("week".to_string()));
    }

    #[test]
    fn test_domain_stats_params() {
        let params = DomainStatsParams {
            period: Some("month".to_string()),
        };
        assert_eq!(params.period, Some("month".to_string()));
    }

    #[test]
    fn test_domain_analytics_params_default() {
        let params = DomainAnalyticsParams::default();
        assert_eq!(params.period, None);
    }

    #[test]
    fn test_domain_analytics_serialization() {
        let data = DomainAnalytics {
            date: "2024-01-01".to_string(),
            sent: 100.0,
            delivered: 95.0,
            opened: 50.0,
            clicked: 25.0,
            bounced: 5.0,
            complained: 1.0,
        };

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("2024-01-01"));
        assert!(json.contains("100"));
    }
}
