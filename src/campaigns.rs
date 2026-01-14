use crate::client::{Client, Result};
use crate::models::*;

pub struct CampaignsClient<'a> {
    client: &'a Client,
}

impl<'a> CampaignsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, payload: &CampaignCreate) -> Result<CampaignCreateResponse> {
        self.client.post("/campaigns", payload).await
    }

    pub async fn get(&self, campaign_id: &str) -> Result<Campaign> {
        self.client.get(&format!("/campaigns/{}", campaign_id)).await
    }

    pub async fn schedule(
        &self,
        campaign_id: &str,
        payload: &CampaignSchedule,
    ) -> Result<CampaignScheduleResponse> {
        self.client
            .post(
                &format!("/campaigns/{}/schedule", campaign_id),
                payload
            ).await
    }

    pub async fn pause(&self, campaign_id: &str) -> Result<CampaignActionResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client
            .post(&format!("/campaigns/{}/pause", campaign_id), &empty).await
    }

    pub async fn resume(&self, campaign_id: &str) -> Result<CampaignActionResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client
            .post(&format!("/campaigns/{}/resume", campaign_id), &empty).await
    }

    /// List all campaigns
    pub async fn list(&self) -> Result<Vec<CampaignListItem>> {
        self.client.get("/campaigns").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campaigns_paths() {
        let paths = vec![
            "/campaigns",
            "/campaigns/test-id",
            "/campaigns/test-id/schedule",
            "/campaigns/test-id/pause",
            "/campaigns/test-id/resume",
        ];

        for path in paths {
            assert!(path.starts_with("/campaigns"));
        }
    }

    #[test]
    fn test_campaign_create() {
        let req = CampaignCreate::new(
            "Newsletter".to_string(),
            "newsletter@example.com".to_string(),
            "Weekly Updates".to_string(),
            "book-id".to_string(),
        );
        assert_eq!(req.name, "Newsletter");
        assert_eq!(req.from, "newsletter@example.com");
        assert_eq!(req.subject, "Weekly Updates");
        assert_eq!(req.contact_book_id, "book-id");
    }

    #[test]
    fn test_campaign_schedule() {
        let req = CampaignSchedule::new();
        assert_eq!(req.scheduled_at, None);
        assert_eq!(req.batch_size, None);
    }
}
