I translated `reverse_string` into `src/lib.rs`, and the test suite passes (1 passed, 0 failed). I ran it once.

The function returns a new reversed `String` instead of reversing in place, as the API in `src/lib.rs` requires. It reverses by characters, which is the same as reversing bytes for ASCII-only input. It also cuts the input at the first NUL character, because the C `strlen` stops there.

My first attempt to write the file through the shell was blocked by the tool, so I wrote it with the file editor instead. This did not affect the result.

No concerns.