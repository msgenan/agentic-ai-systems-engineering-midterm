I translated `original.c` into `src/lib.rs`, and the visible test suite passes (1 test, 1 passed; `./run_tests.sh` run once).

**What the translation does**
- It follows the C parser's structure: expression, term and primary functions with the same grammar, including unary minus and the `%` operator.
- Like the C code, it keeps parsing after the first error and reports the position where parsing finally stopped. The first error recorded wins, so error kind and offset match the C behaviour.
- Input is treated as bytes, and parsing stops at the first NUL, as a C string would. Whitespace uses the C-locale set (space, tab, newline, vertical tab, form feed, carriage return).
- No `unsafe` and no new dependencies.

**Concerns**
- **Overflow:** The C comment says inputs never overflow 64 bits. I used wrapping arithmetic so Rust never panics. For inputs that do overflow, the C code is undefined behaviour, so there is no exact C result to match.
- **Error-path check:** I did not compile or run the C original to compare outputs on edge cases such as `1/0+ )`. The only verification is the single visible test, so the other error-position behaviour is unverified against C.
- **Sandbox edit block:** My first attempt to rename an internal enum in place with `sed` was blocked by a permission prompt. I rewrote the file with the edit tool instead, and the final file contents are what the tests ran against.