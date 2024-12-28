use anyhow::Result;
use rusqlite::Connection;

use crate::core::collection::model::Collection;
use crate::core::note::model::Note;

use super::{embedded_database_path, BackendExt, Id};

pub struct SqliteBackend {
    db_conn: Connection,
}

impl SqliteBackend {
    pub fn new() -> Result<Self> {
        let edb = embedded_database_path()?;
        let db_conn = Connection::open(edb)?;

        Ok(SqliteBackend { db_conn })
    }

    pub async fn install(&self) -> Result<()> {
        self.db_conn.execute(
            r#"CREATE TABLE collections (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            )"#,
            (),
        )?;

        self.db_conn.execute(
            r#"CREATE TABLE notes (
                id INTEGER PRIMARY KEY,
                body TEXT NOT NULL,
                collection_id INTEGER NOT NULL,
                FOREIGN KEY(collection_id) REFERENCES collections(id)
            )"#,
            (),
        )?;

        Ok(())
    }
}

impl BackendExt for SqliteBackend {
    async fn create_collection(&self, name: &str) -> Result<()> {
        self.db_conn
            .execute("INSERT INTO collections (name) VALUES (?1)", (&name,))?;

        Ok(())
    }

    async fn list_collections(&self) -> Result<Vec<Collection>> {
        let mut stmt = self.db_conn.prepare("SELECT id, name FROM collections")?;

        let coll_iter = stmt.query_map([], |row| {
            Ok(Collection {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        Ok(coll_iter.flatten().collect::<Vec<Collection>>())
    }

    async fn create_note(&self, collection_id: &Id, note: String) -> Result<()> {
        self.db_conn.execute(
            "INSERT INTO notes (body, collection_id) VALUES (?1, ?2)",
            (&note, collection_id),
        )?;

        Ok(())
    }

    async fn list_notes(&self, collection_id: &Id) -> Result<Vec<Note>> {
        let mut stmt = self
            .db_conn
            .prepare("SELECT id, body, collection_id FROM notes WHERE collection_id = ?1")?;

        let coll_iter = stmt.query_map([collection_id], |row| {
            Ok(Note {
                id: row.get(0)?,
                body: row.get(1)?,
            })
        })?;

        Ok(coll_iter.flatten().collect::<Vec<Note>>())
    }
}
