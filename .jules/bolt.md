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
