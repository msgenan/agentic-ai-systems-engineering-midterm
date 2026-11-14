# Task: port the minishell parsing path from C to Rust

`c_src/` holds a snapshot of the relevant part of a real C project (a 42-school `minishell`):
the lexer, the syntax checker, the parser and the quote/variable expansion, plus the few libft helpers they call.
The executor, builtins, signals and readline are NOT part of the task.

Goal: implement `src/lib.rs` so that, for any input line, the Rust code produces exactly the same
result as the C code. The public API is already declared in `src/lib.rs` with `todo!()` bodies; keep names and
signatures unchanged. Port only what the path needs (for example the C functions `handle_single_quotes`,
`handle_double_quotes` etc. are dead code and are not required).

How behaviour is compared: `c_src/driver.c` shows the C driver (it mirrors `process()` of `main.c` without the
executor) and `src/bin/driver.rs` is the Rust twin that is already written. Both read one input line from stdin and
print a canonical dump (tokens, then commands, or an `ERR <code>` line). The environment used by the driver is
fixed (see the `ENV` array) and `$?` is 7. Quote/escape marker bytes (\x01, \x02, \x03) that the C code inserts
are part of the observable result and must be reproduced exactly, including behaviour that looks like a bug.
You cannot run the C code in this environment; the C sources are the specification.

Run `./run_tests.sh` to execute the visible tests (`tests/visible.rs`, they run the Rust driver on a few lines and
compare with the dump produced by the C driver). A larger hidden differential test will check more inputs.
