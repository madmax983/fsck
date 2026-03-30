**[God Function]**
**Learning:** The clippy --fix attempts to add `const fn` to `#[wasm_bindgen]` functions which breaks compilation, and some errors require manual fixing.
**Action:** Manually fix clippy errors and avoid using clippy --fix blindly on `#[wasm_bindgen]` projects.

**[Extract Experimental Methods]**
**Learning:** `handle_unknown_command` in `src/commands/executor.rs` had grown into a God Function (~130 lines) containing multiple experimental feature commands grouped in an if-else chain. Extracting it to a helper, simplifying strings logic with `matches!` and `split_once`, and returning `Option<CommandResult>` prevents it from getting out of hand.
**Action:** Continually assess whether `match` blocks or experimental command lists in one method can be cleanly extracted. Use early returns and `Option` whenever checking for a match and short-circuiting.

**[Flatten BASIC Evaluation]**
**Learning:** `evaluate_basic_statement` in `src/commands/executor.rs` processes multiple different BASIC instructions via sequential `if stmt.starts_with(...)` blocks, some of which are quite long (e.g. `GOTO`). This is a classic "Pyramid of Doom" and "God Function" smell.
**Action:** When a method checks multiple text prefixes to dispatch commands, extract the specific logic for each prefix into isolated helper methods. Pass the execution context as a single struct to reduce argument count.
