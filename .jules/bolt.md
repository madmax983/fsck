
**[Clippy misleading `missing_const_for_fn` on String]
**Learning:** Clippy may suggest making a function `const` even if it takes or returns a `String`. On stable Rust, `String` has a destructor which cannot be evaluated at compile-time, resulting in a compilation error if `const` is added.
**Action:** Ignore the `missing_const_for_fn` lint using `#[allow(clippy::missing_const_for_fn)]` instead of blindly applying the `const` modifier to functions handling heap-allocated types with destructors.
