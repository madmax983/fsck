**[Zero-Allocation Padding with Display]**
**Learning:** Passing `format_args!()` to another macro (like `writeln!`) does not respect width/padding format specifiers (e.g., `{:<21}`) natively without intermediate allocation if multiple elements are being formatted together. It also produces `unused_format_specs` warnings in Clippy.
**Action:** To apply padding to a dynamically constructed string without allocating an intermediate `String` (e.g., IP and Port), calculate the string length directly and apply manual padding using `write!` and a loop for spaces, or implement a custom `Display` struct.

**[Cow Accessor Optimization]
**Learning:** Accessor methods that return an owned `String` (e.g., by eagerly calling `.into_owned()` on internal data) force a heap allocation for all consumers, even those that only need to read a borrowed `&str` slice.
**Action:** To avoid unnecessary allocations, expose accessors that return `Cow<'_, str>` instead of `String`. Allow the caller to use the borrowed slice for read-only operations and only call `.into_owned()` when mutation is explicitly required.
**[Clippy cast_lossless on counters]
**Learning:** Initializing a counter with `let mut count = 0;` and later casting it with `count as f64` causes Rust to infer the type as `i32`. This triggers Clippy's `cast-lossless` lint because `i32` to `f64` can be done infallibly using `f64::from()`.
**Action:** Explicitly declare the counter's type as `usize` (e.g., `let mut count: usize = 0;`) if you intend to cast it to a float. Converting `usize` to `f64` is lossy and correctly necessitates the `as f64` cast, satisfying Clippy.
