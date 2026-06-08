**[Extracting Primitives to Drop Borrows]**
**Learning:** Holding an immutable borrow (like `file.name()`) prevents mutable borrows of the parent struct (`self`). Cloning the borrowed value (`.to_string()`) is a common anti-pattern to bypass this.
**Action:** Instead of allocating a `String`, eagerly compute and extract the required primitive data (e.g., boolean flags or `depth_increase` integers) from the `&str` within an isolated block. This drops the immutable borrow early (via NLL), allowing subsequent mutable calls on `self` without allocations.
