I translated `original.c` into `src/lib.rs`, but I could not run the tests. I have no tool that can execute `./run_tests.sh` in this session, so the code is uncompiled and untested.

**What I did**
- Implemented `Table` as an open-addressing hash table with linear probing and tombstones. Slots are an enum (`Empty`, `Used(String, i32)`, `Deleted`).
- Kept the C logic as it was: the same `hash` function using wrapping `u32` arithmetic, and a starting capacity of 8.
- Kept the grow rule `(used+1)*10 > cap*7` (capacity doubles), the tombstone reuse in `put`, and the `len`/`used` counters.
- Kept the C resize behaviour: it re-inserts each live key through `put`.
- `keys()` returns the live keys in slot order.
- Keys are cut at the first NUL byte, because C strings end there. This is a fidelity choice I made.
- No `unsafe` and no new dependencies.

**Concerns**
- Because nothing was compiled or run, there may be compile errors or behaviour differences. Run `./run_tests.sh` to check.
- `ht_keys` took a `max` argument, but the Rust API has none, so `keys()` always returns every live key.