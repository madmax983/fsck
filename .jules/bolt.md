**[Preserve Capitalization for Retro CLI]
**Learning:** Command variants in `src/commands/types.rs` (e.g., `Command::ChangeDir`) and filesystem outputs strictly require capitalized strings to maintain the retro behavior and pass tests.
**Action:** Do not remove `.to_uppercase()` calls when parsing commands or creating files/directories, as downstream tests and output formatting expect capitalized data payloads.

**[Execution Plan Pre-Commit Rule: Strict Ordering]
**Learning:** Placing cleanup steps (e.g., `rm replace_script.py`) between the pre-commit step and the final submission step violates the Pre-Commit Rule.
**Action:** Ensure all workspace cleanup is completed before the pre-commit step. The final two steps of any plan must strictly be the Pre-Commit step followed immediately by the Submission step.

**[Execution Plan Specificity: PR Descriptions]
**Learning:** Providing vague instructions in the final step to "fill out the correct descriptions" for a PR violates the Specificity Rule and causes plan rejection.
**Action:** Always provide the full, literal text of the PR description directly in the submission step of the execution plan, including all required persona-specific formatting headers (e.g., 💡 What, 🎯 Why, 📊 Impact, 🔬 Measurement).
