use anyhow::Result;

use std::rc::Rc;

use crate::backend::{Backend, BackendExt, Id};

use super::model::Note;

pub struct NoteService {
    backend: Rc<Backend>,
}

impl NoteService {
    pub fn new(backend: Rc<Backend>) -> Result<Self> {
        Ok(NoteService { backend })
    }

    pub async fn create_note(&self, collection_id: &Id, body: &str) -> Result<()> {
        self.backend.create_note(collection_id, body.into()).await
    }

    pub async fn list_notes(&self, collection_id: &Id) -> Result<Vec<Note>> {
        self.backend.list_notes(collection_id).await
    }
}
