use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::api::policies::models::{Notify};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyDto {
    pub name: String,
    pub prompt: String,
    #[serde(default)]
    pub notify: Option<Notify>,
    #[serde(default)]
    pub rules: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResponseDto {
    pub id: Uuid,
    pub name: String,
    pub prompt: String,
    pub rules: serde_json::Value,
    pub notify: Option<Notify>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
