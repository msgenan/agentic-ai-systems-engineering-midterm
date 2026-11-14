I translated `original.c` into `src/lib.rs`, but I could not run the tests. The Bash tool is disabled in this session, so `./run_tests.sh` never ran and I have not even confirmed that the code compiles.

**What I did**
- I kept the C algorithm: open addressing with linear probing and tombstones, starting at 8 slots.
- It resizes to double the capacity when `(used+1)*10 > cap*7`. Resizing re-inserts the live entries.
- The `djb2` hash (start value 5381, multiply by 33) uses `wrapping_mul` and `wrapping_add` to match the C `uint32_t` overflow.
- `put` reuses the first tombstone it passes. It only increments `used` when it takes a fresh empty slot, as the C code does.
- `get`, `delete`, `len`, `capacity` and `keys` follow the C versions. `keys` returns live keys in slot order.
- I used `Option<String>` and `Vec`, so there is no `unsafe` and no manual `free`.
- I cut keys at the first NUL character before hashing and comparing. This matches C string behaviour, since a C key ends at the first NUL.

**Concerns**
- Please run `./run_tests.sh` yourself before relying on this.