#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvalError {
    Syntax,
    DivByZero,
}

/// Internal error kind; mirrors the C `EVAL_SYNTAX` / `EVAL_DIV0` codes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ErrKind {
    Syntax,
    DivZero,
}

impl From<ErrKind> for EvalError {
    fn from(e: ErrKind) -> Self {
        match e {
            ErrKind::Syntax => EvalError::Syntax,
            ErrKind::DivZero => EvalError::DivByZero,
        }
    }
}

struct Parser<'a> {
    /// Input bytes, truncated at the first NUL to mirror C's string semantics.
    s: &'a [u8],
    pos: usize,
    err: Option<ErrKind>,
}

impl<'a> Parser<'a> {
    /// Byte at `pos`, or 0 (the C terminator) past the end.
    fn cur(&self) -> u8 {
        self.s.get(self.pos).copied().unwrap_or(0)
    }

    /// Records an error only if none has been recorded yet (first error wins).
    fn fail(&mut self, e: ErrKind) {
        if self.err.is_none() {
            self.err = Some(e);
        }
    }

    fn skip_ws(&mut self) {
        while is_space(self.cur()) {
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
                self.fail(ErrKind::DivZero);
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
                self.fail(ErrKind::Syntax);
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
        self.fail(ErrKind::Syntax);
        0
    }
}

/// C-locale `isspace`.
fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}

/// Evaluates an integer expression with + - * / % ( ) and unary minus.
/// Returns Ok(value), or Err((error, offset where parsing stopped)) with the same
/// error code and offset as the C version.
pub fn eval(s: &str) -> Result<i64, (EvalError, usize)> {
    let bytes = s.as_bytes();
    // C strings end at the first NUL byte.
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let mut p = Parser {
        s: &bytes[..end],
        pos: 0,
        err: None,
    };
    let v = p.parse_expr();
    p.skip_ws();
    if p.err.is_none() && p.cur() != 0 {
        p.fail(ErrKind::Syntax);
    }
    match p.err {
        Some(e) => Err((e.into(), p.pos)),
        None => Ok(v),
    }
}
