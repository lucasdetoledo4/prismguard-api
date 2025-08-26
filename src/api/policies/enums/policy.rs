use serde_repr::{Deserialize_repr, Serialize_repr};

/// Lifecycle events that can trigger notifications (wire-encoded as u8).
#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum JobEvent {
    Queued = 1,
    Running = 2,
    Completed = 3,
    Failed = 4,
    PolicyViolation = 5,
}

/// Webhook target kinds (wire-encoded as u8).
#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum WebhookType {
    Slack = 1,
    Teams = 2,
    Discord = 3,
    Http = 255,
}
