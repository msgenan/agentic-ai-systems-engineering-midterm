The Rust translation is in `src/lib.rs` and the visible test suite passes (1 test, 1 run).

**What I did**
- Translated `original.c` into `src/lib.rs`, keeping the public API the stub declared.
- Slots are an enum (`Empty`, `Used { key, value }`, `Deleted`) instead of the C `state` integer. The probing, tombstone reuse, and load-factor logic follow the C code step by step.
- The hash is DJB2 with `u32` wrapping arithmetic, matching the C overflow behaviour.
- `ht_put` checks the resize threshold before probing, even when it is only updating an existing key, as the C code does. Resize re-inserts live entries through `put`, as in C.
- `ht_free` has no Rust counterpart because `Drop` handles cleanup. There is no `unsafe` and no dependency was added.

**Test status**
- `./run_tests.sh`: 1 passed, 0 failed (observed in one run).

**Concerns**
- The visible test is small. I did not compare the Rust output against the compiled C on random operation sequences, so equivalence on capacity growth, tombstone-heavy workloads, and key order is by inspection only.
- `keys()` returns owned `String`s, not borrowed `&str` as the C `ht_keys` does with `const char **`. This is a deliberate change to fit the existing API.
- The C code would loop forever if the table ever had no empty slot. The load-factor check prevents that, and the Rust port keeps the same check.