**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.

**[Clippy: Needless Lifetimes]
**Learning:** Explicitly declaring and assigning lifetimes in function signatures (e.g., `fn parse<'a>(content: &'a str) -> Result<..., &'a str>`) when the compiler can automatically infer them triggers the `clippy::needless_lifetimes` lint, which fails the build under `-D warnings`.
**Action:** Rely on Rust's lifetime elision rules and omit explicit lifetime annotations whenever possible.
