use async_trait::async_trait;
use uuid::Uuid;

use crate::api::policies::models::Policy;
use crate::error::AppResult;

#[async_trait]
pub trait PolicyRepository: Send + Sync + 'static {
    async fn create(&self, p: Policy) -> AppResult<Policy>;
    async fn get(&self, id: Uuid) -> AppResult<Option<Policy>>;
    async fn list(&self, limit: u32, offset: u32) -> AppResult<Vec<Policy>>;
    async fn update(&self, p: Policy) -> AppResult<Policy>;
    async fn delete(&self, id: Uuid) -> AppResult<bool>;
}
