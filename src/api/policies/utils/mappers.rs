use time::OffsetDateTime;
use uuid::Uuid;

use crate::api::policies::{
    dtos::{CreatePolicyDto, PolicyResponseDto},
    models::{Notify, Policy, Webhook},
};

/// DTO (Notify) -> Model (Notify)
pub fn map_notify_in(dto: &Notify) -> Notify {
    Notify {
        on: dto.on.clone(),
        targets: dto.targets.iter().map(|t| Webhook {
            kind: t.kind.clone(),
            url: t.url.clone(),
            secret: t.secret.clone(),
        }).collect(),
    }
}

/// Model (Notify) -> DTO (Notify)
pub fn map_notify_out(model: &Notify) -> Notify {
    Notify {
        on: model.on.clone(),
        targets: model.targets.iter().map(|t| Webhook {
            kind: t.kind.clone(),
            url: t.url.clone(),
            secret: t.secret.clone(),
        }).collect(),
    }
}

/// Model (Policy) -> DTO (PolicyResponseDto)
pub fn policy_to_response(policy: Policy) -> PolicyResponseDto {
    PolicyResponseDto {
        id: policy.id.unwrap_or(Uuid::nil()),
        name: policy.name,
        prompt: policy.prompt,
        rules: policy.rules,
        notify: policy.notify.as_ref().map(map_notify_out),
        created_at: policy.created_at,
        updated_at: policy.updated_at,
    }
}

/// DTO (CreatePolicyDto) -> Model (Policy)
pub fn request_to_policy(req: CreatePolicyDto) -> Policy {
    Policy {
        id: None,
        name: req.name,
        prompt: req.prompt,
        rules: req.rules,
        notify: req.notify.as_ref().map(map_notify_in),
        created_at: OffsetDateTime::UNIX_EPOCH,
        updated_at: OffsetDateTime::UNIX_EPOCH,
    }
}
