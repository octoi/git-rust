use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
    // Initialize new repository
    Init,

    // Provide content or type and size information for repository objects 
    CatFile {
        #[arg(short)]
        pretty_print: bool,
        hash: String,
    },

    // Compute object ID and optionally creates a blob from a file
    HashObject {
        // Write the object to the database after hashing
        #[arg(short)]
        write: bool,

        // File to hash
        file: PathBuf,
    }
}
