use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::api::policies::enums::{JobEvent, WebhookType};
use super::notifications::{NotificationEvent};

#[derive(Debug, Clone)]
pub struct Policy {
    pub id: Uuid,
    pub name: String,
    pub prompt: String,
    pub rules: Value,
    pub notify: Option<NotificationEvent>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
