**[Accumulator in take_while]
**Learning:** When using `take_while` to limit an iterator based on a cumulative threshold (like maximum byte length), evaluating the individual item's size against the limit instead of an accumulated total will cause the iterator to erroneously consume the entire collection.
**Action:** Declare an accumulator variable outside the closure (e.g., `let mut current_len = 0;`), update it inside the `take_while` block, and evaluate the accumulator against the threshold for correct boundary constraints.
