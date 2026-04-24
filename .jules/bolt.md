**[Zero-Allocation Padding with Display]**
**Learning:** Passing `format_args!()` to another macro (like `writeln!`) does not respect width/padding format specifiers (e.g., `{:<21}`) natively without intermediate allocation if multiple elements are being formatted together. It also produces `unused_format_specs` warnings in Clippy.
**Action:** To apply padding to a dynamically constructed string without allocating an intermediate `String` (e.g., IP and Port), calculate the string length directly and apply manual padding using `write!` and a loop for spaces, or implement a custom `Display` struct.
**[Eliminate formatting macro allocation for newlines]**
**Learning:** `format!("{string}\n")` forces a new heap allocation and copies the contents of an existing string buffer merely to append a newline, degrading performance in hot paths like terminal output streams.
**Action:** When a function already returns an owned `String`, declare it as `mut` and use `string.push('\n')` to append the newline in-place. This is a zero-cost abstraction that modifies the existing buffer's capacity and length directly without a secondary heap allocation.
