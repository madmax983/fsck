# Forge's Journal

**[Refactored God Functions]
**Learning:** Functions like `handle_nova_commands` in `src/commands/executor.rs` and `from_input` in `src/commands/types.rs` have been successfully refactored from nested `if/else if` chains into flattened `match` expressions using guard clauses.
**Action:** Avoid re-refactoring these functions for nesting. Apply this same `match` pattern to other command parsers or deeply nested `if/else if` chains in the future.

**[Performance Optimization Style]
**Learning:** Replacing `.to_uppercase()` with a manually constructed `String::with_capacity` loop still creates a heap allocation and fails the zero-cost abstraction goal. Achieve true zero-allocation by executing comparisons directly against the borrowed `&str` reference using methods like `.eq_ignore_ascii_case()`, bypassing the need for intermediate string buffers entirely.
**Action:** When flattening `if/else if` chains on string matches, use `match c { c if c.eq_ignore_ascii_case("FOO") => ... }` guards to preserve zero-allocation optimizations while flattening the structure.

**[Feature-Gated Module Calls]
**Learning:** Refactoring feature-gated `if / else if` chains into `match` expressions without transferring the `#[cfg(feature = "...")]` attributes to the corresponding match arms silently drops conditional compilation logic, causing critical regressions.
**Action:** When converting feature-gated logic to `match` statements, meticulously preserve all `#[cfg(...)]` attributes by applying them directly above each respective match arm.

**[Extract God Function logic in Command::from_input]**
**Learning:** `from_input` inside `src/commands/types.rs` was a God Function because it contained deep nesting and complex `if / else if` string formatting logic for multiple command paths.
**Action:** Extract specific formatting logic into a flattened `match` statement using guard clauses to preserve zero-allocation checking and improve overall readability.
