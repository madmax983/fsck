**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.
**[Const fn for Static Arrays]
**Learning:** When refactoring functions to return statically-sized arrays of literals (e.g., `[(&'static str, &'static str); N]`) instead of dynamically allocated `Vec`s, `cargo clippy` will trigger the `missing_const_for_fn` lint.
**Action:** Always declare functions returning static arrays of constants as `const fn` to satisfy Clippy and allow the compiler to evaluate them at compile time.
