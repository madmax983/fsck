**Nested Options/Conditionals**
**Learning:** `cmd.splitn(2, ' ').collect::<Vec<&str>>()` followed by length checks inside `if parts.len() == 2` then nesting `if !query.is_empty()` is a common anti-pattern that creates pyramids of doom and unnecessary heap allocations.
**Action:** Use `if condition && let Some((_, arg)) = cmd.split_once(' ')` combined with an early return `if arg.trim().is_empty() { return None; }` to flatten the structure and satisfy clippy lints like `clippy::collapsible_if`.
