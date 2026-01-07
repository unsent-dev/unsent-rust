// Manual helper types and type aliases for SDK functionality
// This file contains types not auto-generated from OpenAPI spec

pub use crate::types::*;
use serde::{Deserialize, Serialize};

// Type aliases for better readability
pub type ApiKey = GetApiKeys200ResponseInner;
pub type ApiKeyCreate = CreateApiKeyRequest;
pub type ContactBookCreate = CreateContactBookRequest;
pub type ContactBook = CreateContactBook200Response;
pub type ContactBookDetails = GetContactBook200Response;
pub type ContactBookUpdate = UpdateContactBookRequest;
pub type ContactBookUpdateResponse = UpdateContactBook200Response;
pub type ContactBookDeleteResponse = DeleteContactBook200Response;
pub type TemplateCreate = CreateTemplateRequest;
pub type Template = GetTemplates200ResponseInner;
pub type TemplateCreateResponse = CreateTemplate200Response;
pub type TemplateUpdate = UpdateTemplateRequest;
pub type Domain = GetDomains200ResponseInner;
pub type DomainCreate = CreateDomainRequest;
pub type Campaign = CreateCampaign200Response;
pub type CampaignListItem = GetCampaigns200ResponseInner;
pub type CampaignCreate = CreateCampaignRequest;
pub type CampaignSchedule = ScheduleCampaignRequest;
pub type ContactCreate = CreateContactRequest;
pub type ContactCreateResponse = CreateContact200Response;
pub type ContactUpdate = UpdateContactRequest;
pub type EmailCreate = SendEmailRequest;
pub type EmailBatchItem = SendEmailRequest;
pub type HealthResponse = GetHealth200Response;
pub type AddSuppressionRequest = crate::types::AddSuppressionRequest;

// Error type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for APIError {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Webhook {
    pub id: String,
    pub url: String,
    pub events: Vec<String>,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebhookCreate {
    pub url: String,
    pub events: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebhookUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebhookId {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebhookDeleteResponse {
    pub success: bool,
}

// Response types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyCreateResponse {
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyDeleteResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Analytics {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesResponse {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReputationResponse {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainCreateResponse {
    #[serde(flatten)]
    pub domain: Domain,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainVerifyResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainDeleteResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailCreateResponse {
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailBatchResponse {
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Email {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailUpdate {
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailUpdateResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailCancelResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailListResponse {
    pub data: Vec<serde_json::Value>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BouncesResponse {
    pub data: Vec<serde_json::Value>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplaintsResponse {
    pub data: Vec<serde_json::Value>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribesResponse {
    pub data: Vec<serde_json::Value>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: i32,
    pub limit: i32,
    pub total: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactUpdateResponse {
    #[serde(rename = "contactId")]
    pub contact_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactUpsert {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactUpsertResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactDeleteResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampaignCreateResponse {
    #[serde(flatten)]
    pub campaign: Campaign,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampaignScheduleResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampaignActionResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateUpdateResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateDeleteResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuppressionListResponse {
    pub data: Vec<serde_json::Value>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuppressionResponse {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuppressionDeleteResponse {
    pub success: bool,
}

// Request option types
#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct EmailListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "domainId", skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ContactListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TimeSeriesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ReputationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SuppressionListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
}
