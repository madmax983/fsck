**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.
**[Cow Lifecycle and Ownership Rules]
**Learning:** `std::borrow::Cow` allows returning a borrowed reference when possible (e.g. `Cow::Borrowed(&str)`) or an owned copy (e.g. `Cow::Owned(String)`). If downstream callers expect a mutated, owned `String` and attempt to reassign or modify the value directly without explicitly extracting the owned string (`.into_owned()`), the borrow checker will fail if the lifetimes mismatch.
**Action:** When updating a hot function to return a `Cow` (like `CorruptionEffect::apply`), ensure downstream callers that mutate the output call `.into_owned()` to safely take ownership and prevent lifetime compilation errors.
