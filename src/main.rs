mod tests;

use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "note-manager-RUST")]
#[command(about = "CLI tool for managing notes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        title: String,
        content: String,
    },
    Read {
        title: String,
    },
    Update {
        title: String,
        content: String,
    },
    Delete {
        title: String,
    },
    Search {
        query: String,
    },
    Sync {
        #[command(subcommand)]
        action: SyncAction,
    },
}

#[derive(Subcommand)]
enum SyncAction {
    Fetch { token: String },
    Push { token: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { title, content } => {
            match create_note(&title, &content) {
                Ok(_) => println!("Created note: {}", title),
                Err(e) => eprintln!("Failed to create note: {:?}", e),
            }
        }
        Commands::Read { title } => {
            match read_note(&title) {
                Ok(content) => println!("{}", content),
                Err(e) => eprintln!("Failed to read note: {:?}", e),
            }
        }
        Commands::Update { title, content } => {
            match update_note(&title, &content) {
                Ok(_) => println!("Updated note: {}", title),
                Err(e) => eprintln!("Failed to update note: {:?}", e),
            }
        }
        Commands::Delete { title } => {
            match delete_note(&title) {
                Ok(_) => println!("Deleted note: {}", title),
                Err(e) => eprintln!("Failed to delete note: {:?}", e),
            }
        }
        Commands::Search { query: _ } => {
            eprintln!("Search function not yet implemented.");
        }
        Commands::Sync { action } => {
            match action {
                SyncAction::Fetch { token } => {
                    println!("Fetched notes with token: {}", token);
                }
                SyncAction::Push { token } => {
                    println!("Pushed notes with token: {}", token);
                }
            }
        }
    }
}

// Function to create a note
fn create_note(title: &str, content: &str) -> io::Result<()> {
    let notes_dir = PathBuf::from("notes");
    if !notes_dir.exists() {
        fs::create_dir_all(&notes_dir)?;
    }
    let file_path = notes_dir.join(format!("{}.txt", title));
    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Function to read a note
fn read_note(title: &str) -> io::Result<String> {
    let file_path = PathBuf::from(format!("notes/{}.txt", title));
    fs::read_to_string(file_path)
}

// Function to update a note
fn update_note(title: &str, content: &str) -> io::Result<()> {
    let file_path = PathBuf::from(format!("notes/{}.txt", title));
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Function to delete a note
fn delete_note(title: &str) -> io::Result<()> {
    let file_path = PathBuf::from(format!("notes/{}.txt", title));
    fs::remove_file(file_path)
}

