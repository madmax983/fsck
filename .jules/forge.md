**[Conditional Compilation in CommandExecutor]
**Learning:** The `handle_nova_commands` function in `src/commands/executor.rs` is already fully compiled under `#[cfg(feature = "nova")]`.
**Action:** When refactoring or adding new feature-gated commands inside this function, do not add redundant nested `#[cfg(feature = "nova")]` or `#[cfg(not(feature = "nova"))]` blocks, as they clutter the code.

**[Zero-Allocation String Matching]
**Learning:** Using `command.to_ascii_uppercase()` before a `match` statement creates unnecessary heap allocations. Conversely, large `if / else if` chains using `.eq_ignore_ascii_case()` avoid allocations but drastically reduce readability.
**Action:** Refactor long `if/else if` string-matching chains into a `match` structure using guard clauses (e.g., `match command { c if c.eq_ignore_ascii_case("CMD") => ... }`) to flatten logic while retaining zero-allocation behavior.
**[Zero-Allocation String Matching]
**Learning:** Using `command.to_ascii_uppercase()` before a `match` statement creates unnecessary heap allocations. Conversely, large `if / else if` chains using `.eq_ignore_ascii_case()` avoid allocations but drastically reduce readability.
**Action:** Refactor long `if/else if` string-matching chains into a `match` structure using guard clauses (e.g., `match command { c if c.eq_ignore_ascii_case("CMD") => ... }`) to flatten logic while retaining zero-allocation behavior. Be careful to preserve inner `#[cfg(feature = "nova")]` compilation boundaries.
