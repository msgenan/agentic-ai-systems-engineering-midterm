I translated `is_prime` into `src/lib.rs`, but I could not verify it. `./run_tests.sh` failed with exit 101 before running any tests: `error: test runner crashed: could not connect to build cache (exit 101)`. I did not run `cargo` directly or try other ways around the failure, as the rules require.

**What I did**
- Replaced the `todo!()` in `src/lib.rs` with a translation of the C code:
  - `n < 2` returns `false`.
  - Trial division runs from `i = 2` while `i * i <= n`, returning `false` on the first divisor found, otherwise `true`.
- I computed `i * i` in `i64`. In the C code, `i * i` can overflow `int` for large `n` (undefined behaviour), so the widening keeps the loop well-defined. For every `n` where the C code is defined, the results match.
- I did not change `Cargo.toml`, add dependencies, or use `unsafe`.

**Test status**
- Not verified. The only test run failed at the build-cache step, before any test executed, so I cannot say whether the tests pass.

**Concerns**
- The failure looks like an infrastructure problem (the script's own message says it is a build-cache connection error), not a problem with the translation. Re-running `./run_tests.sh` once the build cache is reachable should confirm it.
- Since `i * i` is computed in `i64`, the Rust version gives correct results for all `i32` inputs. The C version is undefined for inputs where `i * i` overflows, so the two can differ only in cases where the C code has no defined behaviour.