I translated `is_valid_code` in `src/lib.rs` and the visible test suite passes (1 test, observed on the run).

**What I did**
- The C function checks an anchored POSIX regex, `^[A-Z]{3}-[0-9]{4}$`. The Rust version checks the same thing directly: exactly 8 bytes, three ASCII uppercase letters, a `-`, then four ASCII digits.
- I did not add the `regex` crate, since the task rules say no new dependencies, and the pattern is simple enough to write by hand.
- The C code runs `regexec` on a NUL-terminated string, so it only sees bytes up to the first NUL. I kept that behaviour: for example, `"ABC-1234\0junk"` returns `true`, as it would in C.
- No `unsafe` is used.

**Concerns**
- The NUL-byte handling is deliberate, but it may be surprising for Rust callers. If you'd rather reject strings containing NUL outright, that would be a behaviour change from the C code and needs your decision.
- The test file only covers one valid and one invalid input. I did not add tests, since the rules say to leave the existing tests alone.