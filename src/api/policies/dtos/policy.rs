use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use super::enums::{JobEvent, WebhookType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpointDto {
    pub kind: WebhookType,
    pub url: String,
    pub secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEventDto {
    #[serde(default)]
    pub on: Vec<JobEvent>,
    #[serde(default)]
    pub targets: Vec<WebhookEndpointDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyRequestDto {
    pub name: String,
    pub prompt: String,
    #[serde(default)]
    pub notify: Option<NotificationEventDto>,
    #[serde(default)]
    pub rules: Value,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResponseDto {
    pub id: Uuid,
    pub name: String,
    pub prompt: String,
    pub rules: serde_json::Value,
    pub notify: Option<NotificationEventDto>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
