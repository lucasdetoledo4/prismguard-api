use axum::Router;
use std::sync::Arc;

use crate::api::policies::{
    repositories::mongo::policy_repository::InMemoryPolicyRepository,
    services::policy::PoliciesService,
    routers::policy as policies_router,
};

pub struct AppState {
    pub policies_service: Arc<PoliciesService>,
}

// Holds async for future pool connnections
pub async fn build_router() -> anyhow::Result<Router> {
    // In Memory repo for now
    // Swap to database-specific later
    let repo = Arc::new(InMemoryPolicyRepository::default());
    let policies_service = Arc::new(PoliciesService::new(repo));
    let state = Arc::new(AppState { policies_service });

    let router = Router::new()
        .nest("/v1/policies", policies_router::routes())
        .with_state(state);

    Ok(router)
}
