I translated `is_prime` into `src/lib.rs`, but I couldn't confirm it passes the tests.

- **Test runner:** `./run_tests.sh` exits with code 101 and the message "test runner crashed: could not connect to build cache". It fails before running any tests.
- **Cargo fallback:** I tried `cargo test` directly, but the environment asked for approval and I didn't get it, so it never ran.

The Rust version has the same logic as the C: it returns false for `n < 2`, then trial-divides while `i * i <= n`. I did the arithmetic in `i64` because `i * i` can overflow a 32-bit `int` in C for values near `i32::MAX`, which is undefined behaviour. The `i64` version returns the mathematically correct answer for every `i32`.

If you approve `cargo test`, or fix the build cache so the script works, I can run the checks. The one test in `tests/visible.rs` asserts that 7 is prime and 8 is not.