**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.

**[Missing Documentation in Zero-Cost Abstractions]
**Learning:** Submitting a technically correct zero-cost abstraction without adhering strictly to the persona's explicit documentation rules (e.g., adding `///` comments explaining *why* the optimization matters) results in an incomplete code review.
**Action:** Always include comprehensive `///` doc comments for performance improvements, detailing the eliminated overhead (e.g., heap allocations) and why the zero-cost abstraction matters before requesting a code review.
