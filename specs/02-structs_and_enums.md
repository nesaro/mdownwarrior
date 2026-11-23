The program needs to represent its data using the following structs and enums

```rust
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
```
