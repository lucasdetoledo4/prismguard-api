use std::{collections::BTreeMap, sync::Arc};
use async_trait::async_trait;
use parking_lot::RwLock;
use uuid::Uuid;

use crate::{
    api::policies::{
        models::Policy,
        repositories::interface::PolicyRepository,
    },
    error::{AppError, AppResult},
};


#[derive(Default)]
pub struct InMemoryPolicyRepository {
    inner: Arc<RwLock<BTreeMap<Uuid, Policy>>>,
}

#[async_trait]
impl PolicyRepository for InMemoryPolicyRepository {
    async fn create(&self, policy: Policy) -> AppResult<Policy> {
        let mut inner = self.inner.write();

        if let Some(id) = policy.id {
            if inner.contains_key(&id) {
                return Err(AppError::Conflict("Policy with this ID already exists".into()));
            }

            inner.insert(id, policy.clone());
        }

        Ok(policy)
    }

    async fn get(&self, id: Uuid) -> AppResult<Option<Policy>> {
        Ok(self.inner.read().get(&id).cloned())
    }

    async fn list(&self, limit: u32, offset: u32) -> AppResult<Vec<Policy>> {
        let inner = self.inner.read();
        let mut items: Vec<_> = inner.values().cloned().collect();
        items.sort_by_key(|p| p.created_at);
        let start = offset as usize;
        let end = (start + limit as usize).min(items.len());
        Ok(items.get(start..end).unwrap_or(&[]).to_vec())
    }

    async fn update(&self, policy: Policy) -> AppResult<Policy> {
        let mut inner = self.inner.write();
        
        if let Some(id) = policy.id {
            if !inner.contains_key(&id) {
                return Err(AppError::NotFound("Policy not found".into()));
            }
            inner.insert(id, policy.clone());
        }
        Ok(policy)
    }

    async fn delete(&self, id: Uuid) -> AppResult<bool> {
        Ok(self.inner.write().remove(&id).is_some())
    }
}
