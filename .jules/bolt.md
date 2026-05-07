**[Zero-Allocation Padding with Display]**
**Learning:** Passing `format_args!()` to another macro (like `writeln!`) does not respect width/padding format specifiers (e.g., `{:<21}`) natively without intermediate allocation if multiple elements are being formatted together. It also produces `unused_format_specs` warnings in Clippy.
**Action:** To apply padding to a dynamically constructed string without allocating an intermediate `String` (e.g., IP and Port), calculate the string length directly and apply manual padding using `write!` and a loop for spaces, or implement a custom `Display` struct.

**[Cow Accessor Optimization]
**Learning:** Accessor methods that return an owned `String` (e.g., by eagerly calling `.into_owned()` on internal data) force a heap allocation for all consumers, even those that only need to read a borrowed `&str` slice.
**Action:** To avoid unnecessary allocations, expose accessors that return `Cow<'_, str>` instead of `String`. Allow the caller to use the borrowed slice for read-only operations and only call `.into_owned()` when mutation is explicitly required.

**[Intermediate Vec Allocations]**
**Learning:** Collecting an iterator into a `Vec` solely to calculate derived metrics (like string lengths or word counts) introduces an unnecessary heap allocation.
**Action:** Use iterator combinators like `.fold()`, `.sum()`, or `.count()` directly on the iterator to compute metrics in a single pass without allocating intermediate collections.
**[Avoid Intermediate Allocations with repeat and format]**
**Learning:** When constructing strings that require both repeating a substring and appending other variables, using `format!("{}{}\n", base.repeat(n), count)` triggers multiple heap allocations (one for `.repeat()` and one for `format!`).
**Action:** Pre-calculate the required capacity, instantiate a string with `String::with_capacity()`, use `.push_str()` in a loop for repetitions, and use `writeln!()` to append variables. This eliminates intermediate allocations.
