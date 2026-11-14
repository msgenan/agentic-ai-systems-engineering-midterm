#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvalError {
    Syntax,
    DivByZero,
}

/// Evaluates an integer expression with + - * / % ( ) and unary minus.
/// Returns Ok(value), or Err((error, offset where parsing stopped)) with the same
/// error code and offset as the C version.
pub fn eval(s: &str) -> Result<i64, (EvalError, usize)> {
    // C strings end at the first NUL byte; mirror that.
    let bytes = s.as_bytes();
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let mut p = Parser {
        s: &bytes[..end],
        pos: 0,
        err: None,
    };

    let v = p.parse_expr();
    p.skip_ws();
    if p.err.is_none() && p.peek() != 0 {
        p.set_err(EvalError::Syntax);
    }
    match p.err {
        Some(e) => Err((e, p.pos)),
        None => Ok(v),
    }
}

struct Parser<'a> {
    s: &'a [u8],
    pos: usize,
    /// First error encountered wins, as in the C version.
    err: Option<EvalError>,
}

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

impl<'a> Parser<'a> {
    /// Byte at the current position, or 0 at/after the end (like the C NUL terminator).
    fn peek(&self) -> u8 {
        self.s.get(self.pos).copied().unwrap_or(0)
    }

    fn set_err(&mut self, e: EvalError) {
        if self.err.is_none() {
            self.err = Some(e);
        }
    }

    fn skip_ws(&mut self) {
        while is_space(self.peek()) {
            self.pos += 1;
        }
    }

    fn parse_primary(&mut self) -> i64 {
        self.skip_ws();
        let c = self.peek();
        if c == b'(' {
            self.pos += 1;
            let v = self.parse_expr();
            self.skip_ws();
            if self.peek() != b')' {
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
            while self.peek().is_ascii_digit() {
                v = v
                    .wrapping_mul(10)
                    .wrapping_add((self.peek() - b'0') as i64);
                self.pos += 1;
            }
            return v;
        }
        self.set_err(EvalError::Syntax);
        0
    }

    fn parse_term(&mut self) -> i64 {
        let mut v = self.parse_primary();
        loop {
            self.skip_ws();
            let c = self.peek();
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

    fn parse_expr(&mut self) -> i64 {
        let mut v = self.parse_term();
        loop {
            self.skip_ws();
            let c = self.peek();
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
}
