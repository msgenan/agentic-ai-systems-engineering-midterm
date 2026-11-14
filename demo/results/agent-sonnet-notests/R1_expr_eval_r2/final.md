I translated `original.c` into `src/lib.rs`, but I could not run `./run_tests.sh`, so the result is untested. I have no tool here that can run shell commands, and I did not guess at a result.

- **Translation:** the recursive-descent parser is ported function by function, with the same error and offset behaviour as the C code.
- **Same quirks as C:** parsing carries on after an error is recorded, and only the first error is kept. The end of the string, or an embedded NUL byte, reads as `'\0'`, as it would in C. Whitespace is the C-locale `isspace` set.
- **Overflow:** the C comment says inputs never overflow a 64-bit `long`. I used wrapping arithmetic anyway so the Rust code can't panic where C would silently wrap.
- **Tests:** I read the three assertions in `tests/visible.rs` and traced them against the code by hand. They should pass, but that is not a test run.

Someone should run `./run_tests.sh` to confirm.