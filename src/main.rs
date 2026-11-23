use clap::Parser;
use glob::glob;
use std::path::PathBuf;

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
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mycollection = NoteCollection {
        base_path: "/home/nesaro/wiki/".into(),
        naming_rules: NodeNamingRules::Index,
    };
    let mut pattern: String = String::new();
    pattern.push_str(mycollection.base_path.to_str().expect("reason"));
    pattern.push_str("**/*");
    if let Some(name) = cli.name {
        pattern.push_str(&name.clone());
        pattern.push_str("*");
    };
    pattern.push_str(".md");
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        println!("{}", entry.unwrap().to_str().unwrap());
    }
}
