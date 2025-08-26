use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::api::policies::models::{Policy};
use crate::api::policies::repositories::interface::PolicyRepository;

pub struct PoliciesService {
    repo: Arc<dyn PolicyRepository>,
}

impl PoliciesService {
    pub fn new(repo: Arc<dyn PolicyRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_policy(&self, mut policy: Policy) -> AppResult<Policy> {
        if policy.name.trim().is_empty() {
            return Err(AppError::BadRequest("`name` cannot be empty".into()));
        }

        let now = OffsetDateTime::now_utc();
        policy.id = policy.id.or(Some(Uuid::new_v4()));
        policy.created_at = now;
        policy.updated_at = now;
        self.repo.create(policy).await
    }
}
