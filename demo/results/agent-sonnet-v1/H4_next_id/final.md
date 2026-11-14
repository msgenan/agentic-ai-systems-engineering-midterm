I translated the C code into `src/lib.rs`, and the visible test passes (1 test, 1 run of `./run_tests.sh`).

- **State:** The C `static int next` becomes an `AtomicI32` static, so the code needs no `unsafe`.
- **`next_id`:** It returns the incremented value, as `++next` does in C.
- **`reset_ids`:** It sets the counter back to 0.

**Concerns:**
- Signed overflow is undefined behaviour in C. Here the counter wraps around instead.
- The counter is shared across the whole process. If tests run in parallel threads and share it, they could interfere with each other. The single visible test did not show this.

My first attempt to write the file through a shell redirect needed approval, so I wrote it with the file-edit tool instead.