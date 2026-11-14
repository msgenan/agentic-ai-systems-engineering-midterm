I translated `original.c` into safe Rust in `src/lib.rs`, and the visible test suite passes (1 test, 1 passed, from one `./run_tests.sh` run).

The parser has the same structure as the C one. It reads a NUL byte past the end of the string, and it records only the first error but keeps parsing after it. Error offsets therefore match the C version.

**Concerns:**
- **Wrapping arithmetic:** I used wrapping operations everywhere. The C comment says inputs never overflow, but after an error the parser keeps going and could overflow in cases that C leaves undefined. Wrapping avoids a Rust panic there.
- **Coverage:** The visible tests are the only check I ran. I did not compare error offsets against the compiled C code.