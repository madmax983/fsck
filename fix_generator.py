import re

with open("src/filesystem/generator.rs", "r") as f:
    content = f.read()

content = content.replace(
    "use crate::content::{ContentLibrary, DynamicContent, Era};",
    "use crate::content::{ContentLibrary, DynamicContent, Era, VictimHistory};"
)

content = content.replace(
    "pub fn generate_with_content(seed: u64, initial_depth: u32) -> FilesystemGraph {",
    "pub fn generate_with_content(seed: u64, initial_depth: u32, prev_history: Option<VictimHistory>) -> FilesystemGraph {"
)

content = content.replace(
    """        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut fs = FilesystemGraph::new();
        let library = ContentLibrary::new();""",
    """        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut fs = FilesystemGraph::new();
        let mut library = ContentLibrary::new();
        if let Some(h) = prev_history {
            library.add_history(h);
        }"""
)

content = content.replace(
    "pub fn generate(seed: u64, initial_depth: u32) -> FilesystemGraph {",
    "pub fn generate(seed: u64, initial_depth: u32, prev_history: Option<VictimHistory>) -> FilesystemGraph {"
)

content = content.replace(
    "Self::generate_with_content(seed, initial_depth)",
    "Self::generate_with_content(seed, initial_depth, prev_history)"
)

content = content.replace(
    """            let era = match current_depth {
                3..=8 => Era::Original,
                9..=15 => Era::Technician,
                16..=25 => Era::EstateSale,
                _ => Era::Explorer,
            };""",
    """            let era = match current_depth {
                3..=8 => Era::Original,
                9..=15 => Era::Technician,
                16..=25 => Era::EstateSale,
                26..=30 => Era::Explorer,
                _ => Era::Current,
            };"""
)

with open("src/filesystem/generator.rs", "w") as f:
    f.write(content)
