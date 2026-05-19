**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.

**[Static Arrays vs Vecs]**
**Learning:** Returning `vec![...]` from helper functions creates unnecessary intermediate heap allocations. Since arrays `[T; N]` implement `IntoIterator` in modern Rust, they can be passed directly to methods like `Vec::extend()`.
**Action:** Return statically-sized arrays `[(&'static str, &'static str); N]` instead of `Vec` for fixed compile-time lists to eliminate heap allocations.
