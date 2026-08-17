**[Rust: Feature-Gated Invocations]
**Learning:** Calling a method that is defined with `#[cfg(feature = "x")]` from an ungated code path (e.g., inside an `if/else` chain) without wrapping the *call site* in a matching `#[cfg(feature = "x")]` causes compilation failures when the feature is disabled.
**Action:** Always ensure calls to feature-gated methods are also conditionally compiled at the call site (e.g., placing `#[cfg(feature = "x")]` inside the specific `if` block).
