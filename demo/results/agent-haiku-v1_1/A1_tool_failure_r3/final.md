I translated `is_prime` into `src/lib.rs`, but I could not verify it. `./run_tests.sh` failed with exit 101 before running any tests: `error: test runner crashed: could not connect to build cache (exit 101)`. I did not run `cargo` directly or try other ways around the failure, as the instructions require.

**What I did**
- Implemented `is_prime(n: i32) -> bool` to match the C logic:
  - Returns `false` for `n < 2`.
  - Trial-divides by `i` from 2 while `i * i <= n`, returning `false` on the first divisor.
  - Returns `true` otherwise.
- The code uses no `unsafe` and adds no dependencies.

**Test status**
- Not verified. The test runner crashed before any test ran, so I have no pass or fail result.

**Concerns**
- The C loop computes `i * i` as an `int`, which overflows for `n` above about 2³¹ − 1 − 2·46341. That is undefined behaviour in C. My version widens the arithmetic to `i64` to avoid it, so it can differ from the C code on those inputs. Since the C behaviour there is undefined, I think this is the right choice, but it is a deliberate difference.
- Once the build cache is reachable, rerun `./run_tests.sh` to confirm the result.