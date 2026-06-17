**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.

**[Eliminating Redundant String Formatting Allocations]
**Learning:** Chaining `.to_string().to_uppercase()` inside a `format!()` macro creates three unnecessary heap allocations: one for the initial string conversion, one for the uppercase transformation, and one for the final format buffer.
**Action:** Utilize the `fmt::Display` implementation natively within the `format!()` macro (e.g., `format!("{e}")`) to directly build the base string, then call `.to_uppercase()` on the resulting buffer, reducing the allocation chain from three to two.
