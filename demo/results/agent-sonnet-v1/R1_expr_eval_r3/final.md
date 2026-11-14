I translated `original.c` into `src/lib.rs`, and the visible test suite passes (1 test, 0 failures). I ran `./run_tests.sh` once. That was the only run, because my first attempt was blocked by the shell and I wrote the file with the Write tool instead.

- **Parser:** the recursive-descent logic is the same as the C code. It still keeps parsing after an error is set and reports the first error with the offset where parsing stopped.
- **Embedded NUL:** it ends the input, as it would in a C string.
- **Overflow and division:** arithmetic uses wrapping operations, so odd inputs can't panic. The C comment says inputs never overflow, so this only matters if that promise is broken. Division by zero returns `DivByZero` as before.

The one test I ran is the only check on behaviour. I haven't compared the Rust output against the compiled C on other inputs.