mod models;
mod storage;
mod commands;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "rust-todo")]
#[command(about = "Simple CLI todo app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
        #[arg(short, long)]
        desc: Option<String>,
    },
    List {
        #[arg(short, long)]
        all: bool,
    },
    Done {
        id: String,
    },
    Remove {
        id: String,
    },
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let store_path = storage::default_store_path()?;
    match cli.command {
        Commands::Add { title, desc } => commands::add(&store_path, title, desc)?,
        Commands::List { all } => commands::list(&store_path, all)?,
        Commands::Done { id } => commands::done(&store_path, &id)?,
        Commands::Remove { id } => commands::remove(&store_path, &id)?,
        Commands::Clear => commands::clear(&store_path)?,
    }

    Ok(())
}
