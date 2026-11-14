I translated `is_valid_code` into `src/lib.rs`, and the test suite passes (1 test, 1 passed, from a single run of `./run_tests.sh`).

The C code compiles the pattern `^[A-Z]{3}-[0-9]{4}$` with POSIX regex. I wrote the same check by hand, so no regex crate is needed and `Cargo.toml` is unchanged. A code is valid only if it is exactly 8 bytes: three uppercase ASCII letters, a hyphen, then four ASCII digits.

The C version reads its input as a C string, so it stops at the first NUL byte. The Rust version does the same, cutting the input at the first NUL before checking. A string like `"ABC-1234\0xyz"` is therefore valid in both.

I don't know whether the visible test covers that NUL case, and I haven't compared against the C binary directly.