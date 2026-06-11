**[Implicit Clones via Into Bounds]**
**Learning:** Passing a locally constructed `String` by reference (e.g., `&output`) into a function parameter bounded by `impl Into<String>` or `impl Into<Cow<'static, str>>` silently triggers an implicit `.clone()` to satisfy the trait.
**Action:** Pass locally constructed `String` variables by value (transferring ownership) to `Into`-bounded functions to completely eliminate the silent `.clone()` heap allocation.
