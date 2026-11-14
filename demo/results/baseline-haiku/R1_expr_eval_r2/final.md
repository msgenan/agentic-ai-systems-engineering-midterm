```rust
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvalError {
    Syntax,
    DivByZero,
}

struct Parser<'a> {
    s: &'a [u8],
    pos: usize,
    err: Option<EvalError>,
}

impl<'a> Parser<'a> {
    /// Byte at `pos`, or 0 past the end (mirrors the C string terminator).
    fn cur(&self) -> u8 {
        self.s.get(self.pos).copied().unwrap_or(0)
    }

    /// Records an error only if none has been recorded yet (first error wins).
    fn set_err(&mut self, e: EvalError) {
        if self.err.is_none() {
            self.err = Some(e);
        }
    }

    fn skip_ws(&mut self) {
        // Same set as C `isspace` in the "C" locale.
        while matches!(self.cur(), b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
            self.pos += 1;
        }
    }

    fn primary(&mut self) -> i64 {
        self.skip_ws();
        let c = self.cur();
        if c == b'(' {
            self.pos += 1;
            let v = self.expr();
            self.skip_ws();
            if self.cur() != b')' {
                self.set_err(EvalError::Syntax);
                return 0;
            }
            self.pos += 1;
            return v;
        }
        if c == b'-' {
            self.pos += 1;
            return self.primary().wrapping_neg();
        }
        if c.is_ascii_digit() {
            let mut v: i64 = 0;
            while self.cur().is_ascii_digit() {
                v = v
                    .wrapping_mul(10)
                    .wrapping_add(i64::from(self.cur() - b'0'));
                self.pos += 1;
            }
            return v;
        }
        self.set_err(EvalError::Syntax);
        0
    }

    fn term(&mut self) -> i64 {
        let mut v = self.primary();
        loop {
            self.skip_ws();
            let c = self.cur();
            if c != b'*' && c != b'/' && c != b'%' {
                return v;
            }
            self.pos += 1;
            let r = self.primary();
            if c == b'*' {
                v = v.wrapping_mul(r);
            } else if r == 0 {
                self.set_err(EvalError::DivByZero);
                return 0;
            } else if c == b'/' {
                v = v.wrapping_div(r);
            } else {
                v = v.wrapping_rem(r);
            }
        }
    }

    fn expr(&mut self) -> i64 {
        let mut v = self.term();
        loop {
            self.skip_ws();
            let c = self.cur();
            if c != b'+' && c != b'-' {
                return v;
            }
            self.pos += 1;
            let r = self.term();
            v = if c == b'+' {
                v.wrapping_add(r)
            } else {
                v.wrapping_sub(r)
            };
        }
    }
}

/// Evaluates an integer expression with + - * / % ( ) and unary minus.
/// Returns Ok(value), or Err((error, offset where parsing stopped)) with the same
/// error code and offset as the C version.
pub fn eval(s: &str) -> Result<i64, (EvalError, usize)> {
    // C strings end at the first NUL byte; mirror that behaviour.
    let bytes = s.as_bytes();
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let mut p = Parser {
        s: &bytes[..len],
        pos: 0,
        err: None,
    };
    let v = p.expr();
    p.skip_ws();
    if p.err.is_none() && p.cur() != 0 {
        p.set_err(EvalError::Syntax);
    }
    match p.err {
        Some(e) => Err((e, p.pos)),
        None => Ok(v),
    }
}
```
Notes:
- Mirrors the C parser exactly: first error wins, parsing continues after recoverable errors, and the reported offset is the final parser position.
- Arithmetic uses wrapping ops to avoid panics on overflow (the C comment says inputs never overflow); input is truncated at the first NUL byte to match C string semantics.