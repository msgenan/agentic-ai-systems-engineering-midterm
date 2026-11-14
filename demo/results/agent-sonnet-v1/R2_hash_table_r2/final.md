I translated `original.c` into safe Rust in `src/lib.rs`, and the visible test suite passes (1 test, 0 failures). I ran `./run_tests.sh` once, after the first write attempt was rejected by the tool.

- **Structure:** each slot is an enum with `Empty`, `Used(key, value)` and `Deleted` variants. This is the Rust version of the C `state` field of 0, 1 or 2.
- **Behaviour kept from C:**
  - The hash is the same wrapping-`u32` djb2.
  - Probing is linear, and a put reuses the first tombstone it passed.
  - The table grows when `(used + 1) * 10 > cap * 7`, starting at 8 slots and doubling.
  - Resizing re-inserts the live keys through `put`.
  - `keys()` returns keys in slot order.
- **NUL bytes:** C strings stop at the first NUL byte, so the Rust version truncates keys there before hashing and comparing. The visible test doesn't exercise this. I added it to match what the C code would do.
- **Concerns:** none. The only test run was the visible suite, so I haven't checked edge cases such as heavy deletion followed by resizing.