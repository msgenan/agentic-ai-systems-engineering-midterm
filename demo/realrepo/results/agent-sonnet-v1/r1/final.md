I ported the lexer, syntax check, parser and expansion into `src/lib.rs`. All 12 visible tests pass; that was my only test run, and I could not run the C code to compare against it.

- **Lexer:** it builds tokens and inserts the quote markers (`\x01` for single quotes, `\x02` for double quotes, `\x03` at the closing quote) as the C code does. It returns `None` on an unclosed quote or when no tokens result.
- **Syntax check:** it returns the same error codes, in the same order, as the C code.
- **Parser:** it adds a `\x01` prefix to words that contained a single-quoted part. The C code reads the token after a redirection without checking it exists. My version skips the redirection in that case instead of crashing. The syntax check prevents it anyway.
- **Expansion:** it follows the C `expand_string` logic:
  - `$?` gives the shell's exit status, and `$$` gives an empty string.
  - A `$` not followed by a letter or `_` stays as `$`.
  - `$NAME` takes the first matching environment entry, and a bare `KEY` with no `=` gives an empty string.
  - Arguments after `export` are only expanded if they contain `=`.

I did not port the dead-code quote handlers. The Rust code works on whole characters rather than bytes; every delimiter is ASCII, so the result should be the same, but only the visible tests have checked this.

I did not check how `ft_itoa` handles negative numbers, since `$?` is fixed at 7 in the driver. The Rust code just prints the number normally.