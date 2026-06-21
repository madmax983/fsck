**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.
**[Disjoint Git Merge Diff Blocks]\n**Learning:** Attempting to combine multiple disjoint code modifications into a single   block forces the agent to hallucinate the unverified context lines between the changes, resulting in plan rejection.\n**Action:** When modifying multiple disconnected sections of the same file, use separate, granular  blocks for each contiguous change, strictly targeting only the lines verified during exploration.
**[Disjoint Git Merge Diff Blocks]
**Learning:** Attempting to combine multiple disjoint code modifications into a single `replace_with_git_merge_diff` `SEARCH` block forces the agent to hallucinate the unverified context lines between the changes, resulting in plan rejection.
**Action:** When modifying multiple disconnected sections of the same file, use separate, granular `replace_with_git_merge_diff` blocks for each contiguous change, strictly targeting only the lines verified during exploration.
**[Performance on Cold Paths & Unicode Regressions]
**Learning:** Optimizing error-handling paths (like `Err(e)`) targets 'cold paths' and yields zero measurable performance improvement. Furthermore, swapping `.to_uppercase()` with `.make_ascii_uppercase()` to mutate in-place and avoid intermediate allocations introduces a regression by dropping Unicode awareness.
**Action:** Focus performance optimizations strictly on frequently executed 'hot paths' (e.g., loops, core search functions). When refactoring string allocations, ensure you do not inadvertently alter Unicode behavior to ASCII-only.
