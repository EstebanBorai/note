pub mod collection;

use std::sync::Arc;

use anyhow::Result;

use crate::backend::Backend;

use self::collection::service::CollectionService;

pub struct Note {
    pub collections: CollectionService,
    backend: Arc<Backend>,
}

impl Note {
    pub fn new(backend: Backend) -> Result<Self> {
        let backend = Arc::new(backend);
        let collections = CollectionService::new(Arc::clone(&backend))?;

        Ok(Self {
            collections,
            backend,
        })
    }

    pub async fn install(&self) -> Result<()> {
        self.backend.install().await
    }
}
