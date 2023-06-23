mod indexing;
mod init;
mod search;

use indexing::{add_folder, ls_folders, remove_folder, update};
use init::create_tables;
use search::search;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Eva is an indexing and search tool
#[derive(Debug, Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a folder to the index list
    #[command(arg_required_else_help = true, alias = "a")]
    Add {
        /// Folder to add
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    /// Remove a folder from the index list
    #[command(arg_required_else_help = true, alias = "rm")]
    Remove {
        /// Folder to remove
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    /// List all folders in the index list
    #[command(alias = "l")]
    Ls,
    /// Update all indexes
    #[command(alias = "up")]
    Update,
    /// Search indexes for a given term
    #[command(
        arg_required_else_help = true,
        alias = "find",
        alias = "s",
        alias = "f"
    )]
    Search {
        /// A string to search for
        query: String,
    },
}

fn main() {
    let args = Cli::parse();

    // Create database and tables if they don't exist
    create_tables().ok();

    match args.command {
        Commands::Search { query } => {
            let results = search(&query);
            println!("{}\n", serde_json::to_string_pretty(&results).unwrap());
        }
        Commands::Add { path } => {
            add_folder(&String::from(path[0].to_string_lossy()));
        }
        Commands::Ls => {
            ls_folders();
        }
        Commands::Remove { path } => {
            remove_folder(&String::from(path[0].to_string_lossy()));
        }
        Commands::Update => {
            update();
        }
    };
}
