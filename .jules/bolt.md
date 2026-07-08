**[Zero-Allocation String Building]**
**Learning:** Refactoring a function that returns an allocated `String` into one that takes a `&mut String` buffer parameter successfully avoids intermediate allocations on hot loops (like processing `PRINT` statements in a tight BASIC loop).
**Action:** When a method returns a string built from smaller string chunks (e.g. substrings or parsed parts), check if it's called in a loop. If so, modify the method signature to accept a `&mut String` and append directly to the caller's buffer rather than returning newly allocated strings.
