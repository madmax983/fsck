**[Serialization of VecDeque vs Vec]
**Learning:** While VecDeque is O(1) for pop_front(), changing a serialized Vec to VecDeque in a struct like GameState could break backward compatibility for existing user save data.
**Action:** Optimize Vec operations around the edges (like removing unnecessary clones) rather than changing the fundamental data structure if it's tied to persistent storage.
**[Avoiding O(N) Allocations in String Substrings]
**Learning:** Collecting `.char_indices()` into a `Vec` for string searching causes an unnecessary heap allocation proportional to string size.
**Action:** Use iterator methods directly like `.char_indices().rev().take(n)` and string slicing to find the correct byte offsets instead.
**[Avoiding O(N) Allocations in String Mutation]
**Learning:** Collecting `.chars()` into a `Vec<char>` to mutate a single character and collecting it back into a string causes unnecessary O(N) heap allocations.
**Action:** Use `.char_indices()` to find the byte offset, then construct a new string with `.with_capacity()` using string slices for the prefix and suffix.

**[Avoiding Redundant Allocations with impl Into<String>]**
**Learning:** Passing `&format!(...)` to a constructor that takes `&str` and calls `.to_string()` allocates two `String`s on the heap (one inside `format!`, and another in `.to_string()`).
**Action:** To prevent double heap allocations when a struct stores a `String`, have its constructor methods accept `impl Into<String>` instead of `&str`. This allows callers to pass owned `String`s directly using `.into()`, moving the allocated string instead of allocating a duplicate.

**[Avoiding Heap Allocation in String Repeat]**
**Learning:** Using `String::repeat(n)` inside `.push_str(&" ".repeat(n))` performs an unnecessary heap allocation for the intermediate `String`, which is immediately dropped. Also, in Rust 1.94.0+, `std::iter::repeat(val).take(n)` triggers the `clippy::manual_repeat_n` lint.
**Action:** Use `.extend(std::iter::repeat_n(char, count))` to append repeated characters directly into the existing `String` buffer without creating a temporary `String` or triggering clippy warnings.

**[Returning Static References Instead of Owned Strings]**
**Learning:** To avoid unnecessary heap allocations, return `&'static str` instead of `String` (via `.to_string()`) for hardcoded string literals. Since `&str` implements `Into<String>`, these static slices can be passed directly to APIs accepting `impl Into<String>` without dereferencing or cloning.
**Action:** When a function returns a set of known static strings based on a match condition, set the return type to `&'static str` (or `Option<&'static str>`).

**[Updating String Types in Tests]**
**Learning:** When updating tests after changing a variable's type from `String` to `&'static str`, remove trailing `.as_str()` calls when passing to `.contains()`. Calling `.as_str()` on a `&'static str` can trigger unstable library feature errors (`str_as_str`) in recent Rust versions.
**Action:** Ensure that test assertions correctly compare string references (`&'static str` instead of `&String`) without unnecessary conversions.
