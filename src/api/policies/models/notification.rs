use crate::api::policies::enums::{JobEvent, WebhookType};


#[derive(Debug, Clone)]
pub struct NotificationEvent {
    pub on: Vec<JobEvent>,
    pub targets: Vec<WebhookEndpoint>,
}

#[derive(Debug, Clone)]
pub struct WebhookEndpoint {
    pub kind: WebhookType,
    pub url: String,
    pub secret: Option<String>,
}
