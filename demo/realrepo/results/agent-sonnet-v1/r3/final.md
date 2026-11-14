I couldn't run the tests, so the translation is written but unverified.

I replaced the `todo!()` bodies in `src/lib.rs` with a translation of the C lexer, syntax check, parser and expansion. The Bash tool asked for approval on every `run_tests.sh` invocation, including the plain one by absolute path, and none was granted. That means I haven't compiled it either, so even compile errors are unchecked. That is 0 of 10 test runs used.

What the translation covers:
- **Lexer:** it produces the same tokens, including the `\x01`, `\x02` and `\x03` quote markers and the `quoted` flag. It returns `None` for an unclosed quote or when there are no tokens.
- **Syntax check:** it returns the same error codes, in the same order the C code checks them.
- **Parser:** it splits commands at pipes. It prefixes `\x01` to words that had a single-quoted part. A redirection takes the next token's value as its file, whatever that token's type.
- **Expansion:** `$?`, `$$`, `$` followed by a non-name character, and `$NAME` all follow the C behaviour. For `$NAME` the first matching env entry wins, and a bare `KEY` with no `=` expands to empty. Single-quoted spans are left alone. For an `export` command, arguments without `=` are not expanded and keep their markers.

A human needs to run `./run_tests.sh` (or approve the command) and check the result. I also couldn't run the C code, so the hidden differential test is the only real check against it.