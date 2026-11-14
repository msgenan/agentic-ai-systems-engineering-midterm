I translated `checksum` in `src/lib.rs` and the visible test suite passes (1 test, 0 failed).

The translation loops over the bytes and computes `h = h * 31 + byte` starting from 7. It uses `wrapping_mul` and `wrapping_add` so the `u32` arithmetic wraps on overflow, as the C `uint32_t` does. The code has no `unsafe` and I did not touch `Cargo.toml` or the tests.

There are no concerns. The test checks the empty input and `"ab"` (expected 9832), and both match the C behaviour.