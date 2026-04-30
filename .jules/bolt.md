**[Zero-Allocation Padding with Display]**
**Learning:** Passing `format_args!()` to another macro (like `writeln!`) does not respect width/padding format specifiers (e.g., `{:<21}`) natively without intermediate allocation if multiple elements are being formatted together. It also produces `unused_format_specs` warnings in Clippy.
**Action:** To apply padding to a dynamically constructed string without allocating an intermediate `String` (e.g., IP and Port), calculate the string length directly and apply manual padding using `write!` and a loop for spaces, or implement a custom `Display` struct.

**[Cow Accessor Optimization]
**Learning:** Accessor methods that return an owned `String` (e.g., by eagerly calling `.into_owned()` on internal data) force a heap allocation for all consumers, even those that only need to read a borrowed `&str` slice.
**Action:** To avoid unnecessary allocations, expose accessors that return `Cow<'_, str>` instead of `String`. Allow the caller to use the borrowed slice for read-only operations and only call `.into_owned()` when mutation is explicitly required.
