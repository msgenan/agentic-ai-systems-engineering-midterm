I translated the C hash table into safe Rust in `src/lib.rs`. The visible test suite passed on the one run I made (1 test passed, 0 failed).

- **Slots:** each slot is an enum (`Empty`, `Used(key, value)` or `Deleted`) instead of a state integer. The table starts with 8 slots.
- **Hashing:** the same djb2 hash (seed 5381, multiply by 33) with wrapping `u32` arithmetic.
- **Probing and growth:** linear probing, with the tombstone-reuse rule and the growth check `(used+1)*10 > cap*7` kept exactly as in the C code. As in the C, the growth check also runs when `put` is only updating an existing key.
- **Order:** `keys()` returns live keys in slot order.

One difference from the C: `strcmp` stops at a NUL byte, so C keys can't contain one, while Rust `&str` keys can. This only matters if a key contains `\0`.

I had to write the file with the Write tool because the shell redirect needed approval. That didn't affect the code.