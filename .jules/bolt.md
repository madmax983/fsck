**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.
**[Eliminating Intermediate Allocations in Formatting]
**Learning:** Passing `e.to_string().to_uppercase()` as an argument to `format!()` triggers chained, redundant string allocations (for `.to_string()`, `.to_uppercase()`, and `format!()`).
**Action:** Use the `fmt::Display` trait directly within the format macro (e.g., `let msg = format!("{e}"); msg.to_uppercase()`) to avoid the initial `.to_string()` heap allocation.
