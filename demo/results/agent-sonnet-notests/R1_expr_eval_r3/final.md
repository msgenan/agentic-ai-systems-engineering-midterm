I translated `original.c` into `src/lib.rs`, but I couldn't run the tests. The Bash tool is disabled in this session, so `./run_tests.sh` never ran. The translation is not compiled or tested, and I haven't checked that it builds.

**What I did:**
- **Parser:** I ported the recursive-descent parser (`primary`, `term`, `expr`) function by function, and `eval` now returns the same error code and offset as the C version. Error handling follows the C code: only the first error is recorded, and parsing keeps going after one, so the reported offset matches.
- **Whitespace:** whitespace is the C `isspace` set (space, `\t`, `\n`, `\v`, `\f`, `\r`).
- **NUL bytes:** end of input reads as byte 0, and the input is cut at the first embedded NUL, as C would treat the string.
- **Overflow:** arithmetic uses `wrapping_*` operations so it can't panic. The C comment says inputs never overflow, so this only matters for `i64::MIN / -1`, which is undefined behaviour in C.
- **Unsafe code:** there is none.

**Concern:** I also added a one-line comment inside the `Parser` struct, which has no effect on behaviour.