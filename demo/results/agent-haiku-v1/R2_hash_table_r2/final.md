I translated `original.c` into `src/lib.rs`, keeping the public API as declared. The visible test suite passes (1 test, run once).

What the translation does:
- **Layout:** slots are an enum (`Empty`, `Used { key, value }`, `Deleted`) in a `Vec`. Capacity starts at 8, and `len` and `used` are tracked as in the C code.
- **Hash:** djb2 with `u32` wrapping multiply and add, matching the C `uint32_t` overflow.
- **Put:** the resize check runs before the existing-key check, as in C. Probing reuses the first tombstone and only increments `used` when no tombstone is reused.
- **Resize:** doubles capacity, drops tombstones, and re-inserts live entries in slot order.
- **Keys:** `keys()` returns all live keys in slot order. The C `max` parameter is dropped because the Rust signature has no such argument.

Concerns:
- **NUL bytes in keys:** the C code treats keys as C strings, so anything after an embedded NUL is ignored. I reproduced this by truncating keys at the first `\0`. This is a deliberate choice; say if you'd rather reject such keys.
- **No `unsafe`, no new dependencies.** I also added a `Default` impl, which the API did not ask for.
- **Test coverage is thin:** the single visible test does not exercise resizing, tombstone reuse, or probing past collisions. I checked those paths by reading the code against the C, not by running them.