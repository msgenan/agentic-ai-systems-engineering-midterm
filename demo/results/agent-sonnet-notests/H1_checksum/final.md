I translated `checksum` into `src/lib.rs`, but I couldn't run the tests, so the result is unverified. I have no tool here that can run `./run_tests.sh`.

**What I did:** the C code starts with `h = 7` and computes `h = h*31 + byte` for each byte, with `uint32_t` wrapping on overflow. The Rust version folds over the bytes with `wrapping_mul(31).wrapping_add(b as u32)`, starting from 7. It uses no `unsafe`, and I made no changes to `Cargo.toml` or the tests.

**Concerns:** none that I know of, but please run `./run_tests.sh` yourself to confirm.