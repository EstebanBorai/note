pub mod sqlite;

use std::{fs::create_dir_all, path::PathBuf};

use anyhow::{Error, Result};
use dirs::home_dir;

use crate::core::{collection::model::Collection, note::model::Note};

pub const METADATA_USER_DIR: &str = ".note-app";
pub const METADATA_EDB_FILE: &str = "note.db";

pub fn metadir() -> Result<PathBuf> {
    let mut metadir = home_dir().ok_or(Error::msg("Unable to determinate Home Directory."))?;
    metadir.push(METADATA_USER_DIR);

    Ok(metadir)
}

pub fn embedded_database_path() -> Result<PathBuf> {
    Ok(metadir()?.join(METADATA_EDB_FILE))
}

pub fn install() -> Result<PathBuf> {
    let metadir = metadir()?;

    if metadir.exists() {
        tracing::debug!(metadir=%metadir.display(), "Found existing metadir, aborting install.");
        return Ok(metadir);
    }

    tracing::debug!(metadir=%metadir.display(), "Creating metadir");
    create_dir_all(&metadir)?;
    Ok(metadir)
}

pub enum Backend {
    Sqlite(sqlite::SqliteBackend),
}

impl Backend {
    pub async fn install(&self) -> Result<()> {
        match self {
            Backend::Sqlite(sqlite_backend) => sqlite_backend.install().await,
        }
    }
}

pub type Id = i32;

#[allow(async_fn_in_trait)]
pub trait BackendExt {
    async fn create_collection(&self, name: &str) -> Result<()>;
    async fn list_collections(&self) -> Result<Vec<Collection>>;
    async fn create_note(&self, collection_id: &Id, note: String) -> Result<()>;
    async fn list_notes(&self, collection_id: &Id) -> Result<Vec<Note>>;
}

impl BackendExt for Backend {
    async fn create_collection(&self, name: &str) -> Result<()> {
        match self {
            Backend::Sqlite(sqlite_backend) => sqlite_backend.create_collection(name).await,
        }
    }

    async fn list_collections(&self) -> Result<Vec<Collection>> {
        match self {
            Backend::Sqlite(sqlite_backend) => sqlite_backend.list_collections().await,
        }
    }

    async fn create_note(&self, collection_id: &Id, note: String) -> Result<()> {
        match self {
            Backend::Sqlite(sqlite_backend) => {
                sqlite_backend.create_note(collection_id, note).await
            }
        }
    }

    async fn list_notes(&self, collection_id: &Id) -> Result<Vec<Note>> {
        match self {
            Backend::Sqlite(sqlite_backend) => sqlite_backend.list_notes(collection_id).await,
        }
    }
}
