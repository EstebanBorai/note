use anyhow::Result;
use clap::Subcommand;

use note::backend::Id;
use note::core::NoteApi;

#[derive(Clone, Debug, Subcommand)]
pub enum NotesOpt {
    /// Create a new Note
    New { collection_id: Id, body: String },
    /// List Notes in Collection
    List { collection_id: Id },
}

impl NotesOpt {
    pub async fn exec(&self, api: NoteApi) -> Result<()> {
        match self {
            NotesOpt::New {
                collection_id,
                body,
            } => {
                api.notes.create_note(collection_id, body).await?;
                println!("Note created with success.");
            }
            NotesOpt::List { collection_id } => {
                let notes = api.notes.list_notes(collection_id).await?;

                for note in notes.iter() {
                    println!("{}\t{}", note.id, note.body);
                }

                println!("Found {} note(s).", notes.len());
            }
        }

        Ok(())
    }
}
