I translated `sum_array` into `src/lib.rs`, and the visible test passes (1 passed, 0 failed). I ran `./run_tests.sh` once.

The Rust version converts each `i32` to `i64` and adds them up. I used `i64` because C `long` is 64-bit on this platform (Darwin). It would be 32-bit on Windows, so results could differ there.

My first attempt to write the file with a shell redirect needed approval, so I wrote it with the file-writing tool instead.

I have no concerns beyond the `long` size assumption above.