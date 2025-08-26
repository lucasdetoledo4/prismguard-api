use axum::{routing::post, Json, Router};
use std::sync::Arc;

use crate::app::AppState;
use crate::error::AppResult;

use crate::api::policies::{
    dtos::{CreatePolicyDto, PolicyResponseDto},
    utils::{policy_to_response, request_to_policy},
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", post(create_policy))
}

/// POST /v1/policies
async fn create_policy(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(req): Json<CreatePolicyDto>,
) -> AppResult<Json<PolicyResponseDto>> {
    let policy = request_to_policy(req);
    let saved = state.policies_service.create_policy(policy).await?;
    Ok(Json(policy_to_response(saved)))
}
