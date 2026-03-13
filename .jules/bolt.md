**[Avoiding O(N) Allocations in String Substrings]
**Learning:** Collecting `.char_indices()` into a `Vec` for string searching causes an unnecessary heap allocation proportional to string size.
**Action:** Use iterator methods directly like `.char_indices().rev().take(n)` and string slicing to find the correct byte offsets instead.
