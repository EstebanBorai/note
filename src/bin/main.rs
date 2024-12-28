mod command;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use note::backend::sqlite::SqliteBackend;
use note::backend::{install, Backend};
use note::core::NoteApi;

use self::command::collections::CollectionsOpt;
use self::command::notes::NotesOpt;

#[derive(Debug, Parser)]
#[command(
    name = "note",
    about = "Notes Management System",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/EstebanBorai/note)",
    next_line_help = true
)]
pub enum Command {
    /// New note in the active collection
    New { body: String },
    Collections {
        #[command(subcommand)]
        subcommand: CollectionsOpt,
    },
    Notes {
        #[command(subcommand)]
        subcommand: NotesOpt,
    },
    /// Install the Note Application
    Install,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter_layer)
        .init();

    install()?;

    let args = Cli::parse();
    let backend = Backend::Sqlite(SqliteBackend::new()?);
    let api = NoteApi::new(backend)?;

    match args.command {
        Command::New { body } => {
            println!("Creating new note with body: {}", body);
        }
        Command::Install => {
            install()?;
            api.install().await?;
        }
        Command::Collections { subcommand } => {
            subcommand.exec(api).await?;
        }
        Command::Notes { subcommand } => {
            subcommand.exec(api).await?;
        }
    }

    Ok(())
}
