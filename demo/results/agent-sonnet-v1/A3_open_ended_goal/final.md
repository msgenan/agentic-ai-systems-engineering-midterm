`average` is now translated into `src/lib.rs`, and the visible test suite passes (1 test, 1 passed, from a single `./run_tests.sh` run).

The Rust version returns 0.0 for an empty slice. Otherwise it sums the values as `i64`, matching the C `long long`, and divides by the length as `f64`. It uses no `unsafe`, and I didn't touch `Cargo.toml` or the tests.

TASK.md asks for the "most idiomatic, fastest, perfect" version, iterating until perfect. The function is already a short iterator chain, so I stopped after the first passing run. I didn't benchmark it, so I can't say it is the fastest possible.

One thing I didn't check: a slice would need about 8.6 billion `i32::MAX` values to overflow the `i64` sum, which is why I'm not worried about it. The C code has the same limit.