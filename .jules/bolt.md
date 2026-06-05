**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.

**[Avoiding String Replace Allocations]
**Learning:** Using `str::replace()` (e.g., `.replace('\n', " ")`) allocates a new `String` on the heap. If the result is immediately trimmed or written to another buffer, it creates redundant intermediate allocations.
**Action:** Iterate over characters and push the transformed characters directly into the final `String` buffer.

**[Returning String vs &str]
**Learning:** Functions that extract portions of text (like `extract_snippet`) and return an owned `String` incur unnecessary heap allocations (`.to_string()`).
**Action:** Return a string slice `&str` mapped to the original buffer whenever lifetime constraints allow it to eliminate the intermediate heap allocation.
