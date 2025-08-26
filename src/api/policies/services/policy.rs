use axum::{extract::State, Json};
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::app::AppState;
use crate::error::{AppError, AppResult};

use super::dtos::{
    CreatePolicyRequestDto, NotificationEventDto, PolicyResponseDto, WebhookEndpointDto
};
use super::models::{NotificationEvent, WebhookEndpoint, Policy};

fn notify_from_dto(dto: &Option<NotificationEventDto>) -> Option<NotificationEvent> {
    dto.as_ref().map(|n| NotificationEvent {
        on: n.on.clone(),
        targets: n.targets.iter().map(|t: &WebhookEndpointDto| WebhookEndpoint {
            kind: t.kind,
            url: t.url.clone(),
            secret: t.secret.clone(),
        }).collect(),
    })
}

fn notify_to_dto(n: &Option<NotificationEvent>) -> Option<NotificationEventDto> {
    n.as_ref().map(|nn| NotificationEventDto {
        on: nn.on.clone(),
        targets: nn.targets.iter().map(|t| WebhookEndpointDto {
            kind: t.kind,
            url: t.url.clone(),
            secret: t.secret.clone(),
        }).collect(),
    })
}

fn to_response(p: Policy) -> PolicyResponseDto {
    PolicyResponseDto {
        id: p.id,
        name: p.name,
        prompt: p.prompt,
        rules: p.rules,
        notify: notify_to_dto(&p.notify),
        created_at: p.created_at,
        updated_at: p.updated_at,
    }
}

/// POST /v1/policies
pub async fn create_policy(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePolicyRequestDto>,
) -> AppResult<Json<PolicyResponseDto>> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name cannot be empty".into()));
    }

    let now = now();
    let policy = Policy {
        id: Uuid::new_v4(),
        name: req.name,
        prompt: req.prompt,
        rules: req.rules,
        notify: notify_from_dto(&req.notify),
        created_at: now,
        updated_at: now,
    };

    let saved = state.policies.create(policy).await?;
    Ok(Json(to_response(saved)))
}
