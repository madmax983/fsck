**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.
**[Borrowing Over Allocation in Parsers]
**Learning:** `clippy::needless_lifetimes` is triggered if you explicitly add a lifetime parameter `'a` to a function that can be elided, e.g., `fn parse_basic_program<'a>(content: &'a str) -> ...`. Rust's elision rules automatically tie the output lifetime to the input.
**Action:** When refactoring to use string references `&str` to avoid allocations, omit explicit lifetimes from the function signature and rely on elision.
