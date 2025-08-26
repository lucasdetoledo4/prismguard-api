use serde::{Deserialize, Serialize};

use crate::api::policies::enums::{JobEvent, WebhookType};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notify {
    pub on: Vec<JobEvent>,
    pub targets: Vec<Webhook>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webhook {
    pub kind: WebhookType,
    pub url: String,
    pub secret: Option<String>,
}
