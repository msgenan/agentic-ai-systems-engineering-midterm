I translated `parse_int` into `src/lib.rs` and the visible test suite passes (1 test, `basic`).

**What the translation does**
- Skips leading C-locale whitespace (`' '`, `\t`, `\n`, `\v`, `\f`, `\r`).
- Accepts one optional `+` or `-` sign.
- Reads decimal digits until the first non-digit, ignoring any trailing text. A string with no digits returns 0.
- Works on bytes, so it matches the C `unsigned char` handling for ASCII input.
- No `unsafe` and no new dependencies.

**Concerns**
- **Overflow:** The C code overflows `int` on large inputs, which is undefined behaviour. I used an `i64` accumulator so that `"-2147483648"` (valid under the task's int-range assumption) comes out correctly. The final cast to `i32` truncates out-of-range values rather than panicking. Those inputs are excluded by TASK.md, so this path is not exercised.
- **Edge cases not tested:** The visible suite only covers `"42"` and `"-7"`. I did not add tests for whitespace, `+`, trailing text, empty input or `i32::MIN`, because the rules forbid modifying the test suite.
- **Tool issue:** My first shell command, which combined writing the file and running the tests, was blocked by the sandbox filter. I wrote the file with the file-write tool instead and ran the tests in a separate command. The test result above comes from that separate run.

I used 1 of the 10 allowed test runs.