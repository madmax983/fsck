**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.

**[Avoid format! allocation on static strings and simple concatenations]
**Learning:** `format!("SPEAK: {text}")` allocates a new `String` using `format_args!` which has a larger overhead compared to simply pre-allocating a `String` with known capacity and using `.push_str()`. When this occurs in hot paths (like voice generation which may happen often in some mods), we save allocations. In our case, changing `format!("SPEAK: {text}")` to a pre-allocated `String` prevents an expensive macro expansion allocation sequence.
**Action:** Replace `format!("STATIC: {variable}")` with `String::with_capacity` and `push_str()` when micro-optimizing text output.
