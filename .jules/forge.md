**[Generate Fsck Scan]**
**Learning:** Hardcoded strings in `generate_surface_scan` and other `generate_*_scan` functions make them less flexible and repetitive, returning string outputs in an unstructured format.
**Action:** Extract the scan generation logic to its own module or helper functions to improve readability and enforce idiomatic Rust patterns without changing runtime behavior.

**[Refactor FsckScanGenerator strings]**
**Learning:** Large, multi-line string templates using `format!` can be difficult to read and modify incrementally (e.g., conditionally appending text like `loop_warning`).
**Action:** Replace monolithic `format!` macros with `String::with_capacity` and sequential `writeln!` calls to construct output efficiently and explicitly.

**[Refactor handle_nova_commands]**
**Learning:** A giant match statement where each arm executes significant logic acts as a "God Object" and "Pyramid of Doom", drastically reducing readability and violating DRY/single-responsibility principles. Also, helper functions unconditionally returning `Some(CommandResult)` caused `clippy::unnecessary_wraps` lints.
**Action:** Extract match arms into separate, strongly typed helper functions. Ensure those helper functions directly return the inner type (e.g., `CommandResult`) instead of an unnecessary `Option`, and wrap it in `Some(...)` only at the call site within the match statement.**[Refactor BASIC Execution]**\n**Learning:** Large command executor structs often become "God Objects" by absorbing secondary responsibilities like parsing and interpreting embedded scripting languages.\n**Action:** Extract embedded language interpreters into isolated modules (e.g., ) to flatten structure, reduce cognitive load, and enforce single responsibility.

**[Refactor BASIC Execution]**
**Learning:** Large command executor structs often become "God Objects" by absorbing secondary responsibilities like parsing and interpreting embedded scripting languages.
**Action:** Extract embedded language interpreters into isolated modules to flatten structure, reduce cognitive load, and enforce single responsibility.
