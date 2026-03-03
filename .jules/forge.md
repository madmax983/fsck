**[God Function]**
**Learning:** The clippy --fix attempts to add `const fn` to `#[wasm_bindgen]` functions which breaks compilation, and some errors require manual fixing.
**Action:** Manually fix clippy errors and avoid using clippy --fix blindly on `#[wasm_bindgen]` projects.
