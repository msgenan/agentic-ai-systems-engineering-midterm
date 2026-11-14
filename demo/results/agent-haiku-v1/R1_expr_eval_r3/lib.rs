#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvalError {
    Syntax,
    DivByZero,
}

/// Parser state. Mirrors the C `Parser` struct: the input is treated as a
/// NUL-terminated byte string (reading past the end yields 0), and only the
/// first error is recorded. Parsing does not stop at the first error, so the
/// final offset matches the C version exactly.
struct Parser<'a> {
    s: &'a [u8],
    pos: usize,
    err: Option<EvalError>,
}

impl<'a> Parser<'a> {
    /// Byte at `i`, or 0 (the C string terminator) past the end.
    fn at(&self, i: usize) -> u8 {
        self.s.get(i).copied().unwrap_or(0)
    }

    fn cur(&self) -> u8 {
        self.at(self.pos)
    }

    fn set_err(&mut self, e: EvalError) {
        if self.err.is_none() {
            self.err = Some(e);
        }
    }

    fn skip_ws(&mut self) {
        // C isspace in the "C" locale: space, \t, \n, \v, \f, \r.
        while matches!(self.cur(), b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
            self.pos += 1;
        }
    }

    fn parse_expr(&mut self) -> i64 {
        let mut v = self.parse_term();
        loop {
            self.skip_ws();
            let c = self.cur();
            if c != b'+' && c != b'-' {
                return v;
            }
            self.pos += 1;
            let r = self.parse_term();
            v = if c == b'+' {
                v.wrapping_add(r)
            } else {
                v.wrapping_sub(r)
            };
        }
    }

    fn parse_term(&mut self) -> i64 {
        let mut v = self.parse_primary();
        loop {
            self.skip_ws();
            let c = self.cur();
            if c != b'*' && c != b'/' && c != b'%' {
                return v;
            }
            self.pos += 1;
            let r = self.parse_primary();
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

    fn parse_primary(&mut self) -> i64 {
        self.skip_ws();
        let c = self.cur();
        if c == b'(' {
            self.pos += 1;
            let v = self.parse_expr();
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
            return self.parse_primary().wrapping_neg();
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
}

/// Evaluates an integer expression with + - * / % ( ) and unary minus.
/// Returns Ok(value), or Err((error, offset where parsing stopped)) with the same
/// error code and offset as the C version.
pub fn eval(s: &str) -> Result<i64, (EvalError, usize)> {
    let mut p = Parser {
        s: s.as_bytes(),
        pos: 0,
        err: None,
    };
    let v = p.parse_expr();
    p.skip_ws();
    if p.err.is_none() && p.cur() != 0 {
        p.err = Some(EvalError::Syntax);
    }
    match p.err {
        Some(e) => Err((e, p.pos)),
        None => Ok(v),
    }
}
