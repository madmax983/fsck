**[Clippy unused_doc_comments]**
**Learning:** Using `///` doc-comments inside a function body (e.g., to document an optimization directly above the code block) triggers the `unused_doc_comments` clippy lint because rustdoc does not generate documentation for statements or expressions.
**Action:** Use standard `//` comments for inline code documentation inside function bodies. Only use `///` doc comments for item declarations (functions, structs, traits, etc.) where they can be properly attached by rustdoc.
