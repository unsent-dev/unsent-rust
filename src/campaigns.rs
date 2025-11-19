use crate::client::{Client, Result};
use crate::types::*;

pub struct CampaignsClient<'a> {
    client: &'a Client,
}

impl<'a> CampaignsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self, payload: &CampaignCreate) -> Result<CampaignCreateResponse> {
        self.client.post("/campaigns", payload)
    }

    pub fn get(&self, campaign_id: &str) -> Result<Campaign> {
        self.client.get(&format!("/campaigns/{}", campaign_id))
    }

    pub fn schedule(&self, campaign_id: &str, payload: &CampaignSchedule) -> Result<CampaignScheduleResponse> {
        self.client.post(&format!("/campaigns/{}/schedule", campaign_id), payload)
    }

    pub fn pause(&self, campaign_id: &str) -> Result<CampaignActionResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client.post(&format!("/campaigns/{}/pause", campaign_id), &empty)
    }

    pub fn resume(&self, campaign_id: &str) -> Result<CampaignActionResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client.post(&format!("/campaigns/{}/resume", campaign_id), &empty)
    }
}
