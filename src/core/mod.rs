pub mod collection;
pub mod note;

use std::rc::Rc;

use anyhow::Result;
use note::service::NoteService;

use crate::backend::Backend;

use self::collection::service::CollectionService;

pub struct NoteApi {
    pub collections: CollectionService,
    pub notes: NoteService,
    backend: Rc<Backend>,
}

impl NoteApi {
    pub fn new(backend: Backend) -> Result<Self> {
        let backend = Rc::new(backend);
        let collections = CollectionService::new(Rc::clone(&backend))?;
        let notes = NoteService::new(Rc::clone(&backend))?;

        Ok(Self {
            collections,
            notes,
            backend,
        })
    }

    pub async fn install(&self) -> Result<()> {
        self.backend.install().await
    }
}
