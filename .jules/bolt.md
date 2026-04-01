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

**[Avoiding Vec Allocation in Directory Listing]**
**Learning:** Returning `Vec<String>` from a function that iterates over graph neighbors (like `list_directories`) causes unnecessary heap allocations for both the `Vec` and the copied `String`s when callers only need read access or iteration over string slices.
**Action:** Return an `impl Iterator<Item = &str>` from the method instead. Callers that strictly need an owned vector can use `.map(String::from).collect::<Vec<_>>()`, while callers that only need to iterate avoid allocation entirely.

**[Avoiding Heap Allocation in Random Iterator Elements]**
**Learning:** Using `.collect::<Vec<_>>()` on an iterator just to select a random element via manual index logic (like `slice[rng.gen_range(0..slice.len())]`) creates an unnecessary heap allocation for the intermediate vector.
**Action:** Replace `.collect::<Vec<_>>()` and subsequent index logic with `Iterator::choose(&mut rng)` (from `rand::prelude::*`) to lazily select a random item without allocating memory.
