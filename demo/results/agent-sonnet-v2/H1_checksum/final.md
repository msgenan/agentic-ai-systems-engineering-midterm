`src/lib.rs` now contains the Rust version of `checksum`, and `./run_tests.sh` passes (1 test, 0 failures).

It matches the C code: the hash starts at 7, and for each byte it multiplies by 31 and adds the byte. I used `wrapping_mul` and `wrapping_add` so overflow wraps around like the C `uint32_t` does, instead of panicking in debug builds.

I didn't re-read `TASK.md` beyond the first pass. It is written in Turkish and asks for a Rust translation of `original.c` that fits the API in `src/lib.rs` and behaves the same as the C code. The `checksum(data: &[u8]) -> u32` signature was already in the stub, so I kept it.