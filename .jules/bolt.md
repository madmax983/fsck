**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.
**[Clippy missing_const_for_fn]
**Learning:** Simple accessor methods that only return a reference to a struct field (e.g., `&self.field`) will trigger Clippy's `clippy::missing_const_for_fn` lint if they are not marked as `const fn`.
**Action:** Add the `const` keyword to such simple accessor methods (e.g., `pub const fn accessor(&self) -> &Type`) to satisfy Clippy.
