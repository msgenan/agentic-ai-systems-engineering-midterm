The visible test passes. I stopped the extra differential check because the sandbox kept rejecting the scratch-directory commands, so the C-vs-Rust comparison on edge cases is still unverified.

## Report

**What I did**
- Implemented `eval` in `src/lib.rs` as a faithful port of `original.c`, keeping the same recursive-descent structure: `parse_expr` → `parse_term` → `parse_primary`, with unary minus, `* / %`, and `+ -`.
- Kept the C behaviour that matters for the error offset: parsing does not stop at the first error. Only the first error code is kept, and parsing keeps advancing, so the returned offset is where parsing ended, as in C.
- The input is handled as bytes with a virtual NUL past the end, so a NUL byte ends the expression as it does in C. Whitespace matches C's `isspace` in the C locale, and digits are ASCII only.
- Division or modulo by zero sets `DivByZero` and returns 0, as in C.
- No `unsafe`, no new dependencies.

**Test status**
- `./run_tests.sh`: 1 test passed, 0 failed (observed in the run output). I used 1 of the 10 allowed test runs.

**Concerns**
- **Overflow:** the C code has undefined behaviour on overflow, and the header comment says inputs never overflow. I used `wrapping_*` arithmetic so Rust never panics; for overflowing inputs the results are wrapping results, not a C-defined value. `i64::MIN / -1` wraps to `i64::MIN`.
- **Deep nesting:** very deeply nested parentheses or unary minus can overflow the stack, as in the C version.
- **Unverified:** I did not run a differential comparison against the compiled C on edge cases (for example `1/0+ 2`, `(1))`, `1 2`, `1 ++ 2`). The reasoning above says these match, but that is not confirmed by a run.