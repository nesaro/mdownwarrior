use std::path::PathBuf;
use clap::{Parser, Subcommand};

enum NodeNamingRules {
    Luhmann,
    Timestamp14,
    Timestamp14Description,
    Index,
}

struct NoteCollection {
    base_path: PathBuf,
    naming_rules: NodeNamingRules,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional string to filter on
    name: Option<String>
}

fn main() {
    let cli = Cli::parse();
    let _mycollection = NoteCollection {
        base_path: "/home/nesaro/wiki/".into(),
        naming_rules: NodeNamingRules::Index,
    };
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }
    println!("Hello, world!");
}
