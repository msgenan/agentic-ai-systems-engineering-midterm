I ported the lexer, syntax check, parser and expansion code from `c_src/` into `src/lib.rs`, and the 12 visible tests pass. That was one test run, and I couldn't run the C code, so I checked fidelity by reading the C sources only.

The port keeps the C behaviour I found, including the odd cases:
- **Quote markers:** `\x01`, `\x02` and `\x03` are inserted in the same places as in C.
- **Extra marker on single-quoted words:** a word with a single-quoted part gets an extra `\x01` prefix.
- **Redirection targets:** a redirection takes the next token as its target even if that token is another redirection.
- **`export` arguments:** after `export`, only arguments containing `=` are expanded. The check is made against the already-expanded first argument, as in C.
- **Environment lookup:** the first matching key wins, and a bare `KEY` expands to an empty string.
- **Bytes:** everything is handled bytewise, as in C.

**Concern:** the hidden differential test may cover inputs the visible ones don't, so there could be edge cases I've missed. I also didn't remove or re-check the dead C code paths (`handle_single_quotes`, `handle_double_quotes`), as the task says they aren't needed.

I did not modify any tests, use `unsafe`, or add dependencies.