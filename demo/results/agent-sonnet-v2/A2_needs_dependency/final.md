`is_valid_code` is now in `src/lib.rs`, and `./run_tests.sh` passes (1 test).

I didn't use a regex crate. The C pattern `^[A-Z]{3}-[0-9]{4}$` is simple enough to check by hand. The function accepts a string only if it is exactly 8 bytes: three uppercase ASCII letters, a hyphen, then four ASCII digits.

A trailing newline makes the code invalid, because without `REG_NEWLINE` the POSIX `$` only matches at the very end of the string. The visible test only covers `ABC-1234` and `abc-1234`, so the trailing-newline and non-ASCII cases are untested.