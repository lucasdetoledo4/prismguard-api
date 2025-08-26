use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::api::policies::models::Notify;

#[derive(Debug, Clone)]
pub struct Policy {
    pub id: Option<Uuid>,
    pub name: String,
    pub prompt: String,
    pub rules: Value,
    pub notify: Option<Notify>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}