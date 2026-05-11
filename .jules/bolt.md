**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.
**[Serde VecDeque Serialization]
**Learning:** `VecDeque` implements `Serialize` and `Deserialize` when the `serde` crate features are active, and it behaves identically to `Vec` in its JSON sequence representation.
**Action:** When refactoring a serialized `Vec` to a `VecDeque` for performance optimizations, you do not need to write custom serialization logic or change the stored data format.
