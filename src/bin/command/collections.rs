use anyhow::Result;
use clap::Subcommand;
use note::core::Note;

#[derive(Clone, Debug, Subcommand)]
pub enum CollectionsOpt {
    /// Create a new Collection
    New { name: String },
    /// List Existing Collections
    List,
}

impl CollectionsOpt {
    pub async fn exec(&self, api: Note) -> Result<()> {
        match self {
            CollectionsOpt::New { name } => {
                api.collections.create_collection(name).await?;
                println!("Collection \"{name}\" created with success.");
            }
            CollectionsOpt::List => {
                let colls = api.collections.list_collections().await?;

                for coll in colls.iter() {
                    println!("{}\t{}", coll.id, coll.name);
                }

                println!("Found {} collection(s).", colls.len());
            }
        }

        Ok(())
    }
}
