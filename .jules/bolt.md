**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.
**[Zero-Allocation String Formatting]
**Learning:** Chaining string replacement methods like `.replace('\n', " ").trim()` creates unnecessary intermediate String heap allocations on hot paths.
**Action:** Achieve zero-allocation formatting by avoiding string replacement allocations; instead, iterate over the string (e.g., using `.split_whitespace()`) and push characters or words directly into an existing `&mut String` buffer.
