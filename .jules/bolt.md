**[Eliminating Intermediate Allocations in Error Formatting]**
**Learning:** Formatting an error message using `format!("?{}\n", e.to_string().to_uppercase())` creates up to three intermediate heap allocations (one for `.to_string()`, one for `.to_uppercase()`, and one for `format!`).
**Action:** Use the `fmt::Display` trait directly within the format macro to avoid the initial `.to_string()` allocation (`format!("?{e}\n")`), and use `.make_ascii_uppercase()` on the resulting mutable string to apply casing in-place without another allocation.
