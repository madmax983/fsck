**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.
**[Slice over String for Substrings]
**Learning:** Returning a newly allocated `String` (e.g. `content[start..end].to_string()`) when extracting a substring from an existing `&str` creates unnecessary heap allocation and copying.
**Action:** Return a slice reference (`&str`) to the original data, letting lifetimes ensure safety and achieving zero-allocation substring extraction.
