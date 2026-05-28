**[Handle Nova Commands Refactor]**
**Learning:** `handle_nova_commands` was an extremely long (118 lines) `if/else if/else` block doing string matching for ~24 commands. This "pyramid of doom" created high cognitive load and excessive nesting.
**Action:** Flattened into a single `match (cmd_word, arg)` block utilizing zero-allocation guard clauses (e.g. `(c, a) if c.eq_ignore_ascii_case("SPEAK")`) and explicit literal matching for empty arguments `(c, "")` instead of `a if a.is_empty()`, perfectly retaining existing semantics while improving code clarity dramatically.
