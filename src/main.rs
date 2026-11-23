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

fn main() {
    let _mycollection = NoteCollection {
        base_path: "/home/nesaro/wiki/".into(),
        naming_rules: NodeNamingRules::Index,
    };
    println!("Hello, world!");
}
