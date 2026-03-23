**[Generate Fsck Scan]**
**Learning:** Hardcoded strings in `generate_surface_scan` and other `generate_*_scan` functions make them less flexible and repetitive, returning string outputs in an unstructured format.
**Action:** Extract the scan generation logic to its own module or helper functions to improve readability and enforce idiomatic Rust patterns without changing runtime behavior.

**[Refactor FsckScanGenerator strings]**
**Learning:** Large, multi-line string templates using `format!` can be difficult to read and modify incrementally (e.g., conditionally appending text like `loop_warning`).
**Action:** Replace monolithic `format!` macros with `String::with_capacity` and sequential `writeln!` calls to construct output efficiently and explicitly.
