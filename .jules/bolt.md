**[Optimizing Basic Formatting allocations]**
**Learning:** Returning strings created via `format!` inside loops or helper functions creates multiple intermediate heap allocations that can be costly and increase binary bloat.
**Action:** When executing complex command outputs or appending repeated lines (like in `execute_basic_program`), pass `&mut String` output buffers directly and use `write!` or `writeln!` instead of returning strings and pushing them, and pre-allocate reasonable capacities using `String::with_capacity()`.
