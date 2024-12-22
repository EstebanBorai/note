use anyhow::Result;

use std::sync::Arc;

use crate::backend::{Backend, BackendExt};

use super::model::Collection;

pub struct CollectionService {
    backend: Arc<Backend>,
}

impl CollectionService {
    pub fn new(backend: Arc<Backend>) -> Result<Self> {
        Ok(CollectionService { backend })
    }

    pub async fn create_collection(&self, name: &str) -> Result<()> {
        self.backend.create_collection(name).await
    }

    pub async fn list_collections(&self) -> Result<Vec<Collection>> {
        self.backend.list_collections().await
    }
}
