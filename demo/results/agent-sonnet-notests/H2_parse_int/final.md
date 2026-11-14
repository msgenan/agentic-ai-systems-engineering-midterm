I translated `parse_int` into `src/lib.rs`, but I haven't run the tests. I have no tool here that can run `./run_tests.sh`, so the result is unverified.

The translation matches the C code step by step:
- **Whitespace:** it skips leading space, `\t`, `\n`, `\v`, `\f` and `\r`.
- **Sign:** it accepts one optional `+` or `-`.
- **Digits:** it reads ASCII digits and stops at the first other character. An empty or non-numeric string returns 0.
- **Overflow:** it uses wrapping arithmetic. In C, `-2147483648` overflows during accumulation, which is undefined behaviour. In Rust it wraps and gives the expected result.

The task says inputs stay within the `int` range, so the wrapping only matters for that edge case.

Please run `./run_tests.sh` yourself to confirm.