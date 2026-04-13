**[Generate Fsck Scan]**
**Learning:** Hardcoded strings in `generate_surface_scan` and other `generate_*_scan` functions make them less flexible and repetitive, returning string outputs in an unstructured format.
**Action:** Extract the scan generation logic to its own module or helper functions to improve readability and enforce idiomatic Rust patterns without changing runtime behavior.

**[Refactor FsckScanGenerator strings]**
**Learning:** Large, multi-line string templates using `format!` can be difficult to read and modify incrementally (e.g., conditionally appending text like `loop_warning`).
**Action:** Replace monolithic `format!` macros with `String::with_capacity` and sequential `writeln!` calls to construct output efficiently and explicitly.

**[Refactor handle_nova_commands]**
**Learning:** A giant match statement where each arm executes significant logic acts as a "God Object" and "Pyramid of Doom", drastically reducing readability and violating DRY/single-responsibility principles. Also, helper functions unconditionally returning `Some(CommandResult)` caused `clippy::unnecessary_wraps` lints.
**Action:** Extract match arms into separate, strongly typed helper functions. Ensure those helper functions directly return the inner type (e.g., `CommandResult`) instead of an unnecessary `Option`, and wrap it in `Some(...)` only at the call site within the match statement.
**[Iterator Chains vs Manual Loops]**
**Learning:** Deeply nested `for` loops used for linear searches (e.g., iterating to find a file by name inside a collection) act as a "Pyramid of Doom" and obscure the main logic of the function.
**Action:** Replace manual `for` loops used for searching with idiomatic `Iterator::find(...)` pipelines, coupled with Guard Clauses (early returns using `let Some(...) = ... else { return; }`) to flatten functions and dramatically improve readability.
**[Extracted God Function logic in FilesystemGenerator::add_files]**
**Learning:** `add_files` was a God Function because it contained deep nesting and complex `match` statements for generating multiple different file types inline.
**Action:** Extract specific file generation types into smaller, private helper functions (e.g., `generate_trapdoor_file`, `generate_dynamic_file`) to flatten nesting and improve overall readability of the core generation loops.
**[Refactor God-function Conversion & String Allocations]**
**Learning:** Hardcoded, monolithic depth-to-Era conversion mappings (`match` blocks) inside generator functions make business logic bloated. Also, using `.fold(String::new(), ...)` to construct strings dynamically generates unnecessary heap allocations when the content could be appended directly.
**Action:** Extract generic type conversions into `From<T> / Into<T>` trait implementations to encapsulate mapping logic cleanly out of main execution loops. Replace intermediate string generation in closures by using simple `for` loops writing directly to the target mutable string buffer using `writeln!`.
**[Extracted God Function logic in SystemDiagnostics::generate_report]**
**Learning:** `generate_report` was a God Function because it contained deep nesting and complex `match` statements for generating multiple different report types inline.
**Action:** Extract specific report generation types into smaller, private helper functions (e.g., `generate_surface_report`, `generate_corruption_report`) to flatten nesting and improve overall readability of the core generation loops.
**[Extracted God Function logic in SearchTool::format_match]**
**Learning:** `format_match` inside `src/experimental/grep.rs` was a God Function because it contained deep nesting and complex `match` statements for generating multiple different match formatting logic inline.
**Action:** Extract specific match formatting types into smaller, private helper functions (e.g., `format_surface_match`, `format_corruption_match`) to flatten nesting and improve overall readability of the core generation loops.
