`parse_int` is now translated to Rust in `src/lib.rs`, and `./run_tests.sh` passes (1 test, 0 failures).

It behaves like the C code. It skips leading whitespace (space, `\t`, `\n`, `\v`, `\f`, `\r`) and accepts one optional `+` or `-`. Then it reads digits until the first non-digit and returns the signed value.

I used wrapping arithmetic so an overflow wraps like the C code in practice instead of panicking in debug builds. Your task says inputs stay within `int` range, so this shouldn't come up.