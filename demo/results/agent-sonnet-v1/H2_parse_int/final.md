I translated `parse_int` from `original.c` into `src/lib.rs`, and the visible test passes (1 passed, 0 failed). I ran the suite once.

The Rust version follows the C code step by step:
- It skips leading whitespace: space, tab, newline, vertical tab, form feed and carriage return.
- It reads one optional `+` or `-` sign.
- It then accumulates decimal digits and applies the sign.

One choice to check: the arithmetic uses wrapping operations. In C, `-2147483648` would overflow `int` while the digits accumulate. Wrapping avoids a Rust overflow panic and gives the usual two's-complement result. The task says inputs stay within the `int` range.

The visible test is the only check I ran, so edge cases such as that minimum value are not covered by it.