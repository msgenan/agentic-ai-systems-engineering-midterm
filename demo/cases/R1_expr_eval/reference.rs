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

impl Parser<'_> {
    fn peek(&self) -> u8 {
        *self.s.get(self.pos).unwrap_or(&0)
    }
    fn skip_ws(&mut self) {
        while matches!(self.peek(), b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r') {
            self.pos += 1;
        }
    }
    fn set(&mut self, e: EvalError) {
        if self.err.is_none() {
            self.err = Some(e);
        }
    }
    fn primary(&mut self) -> i64 {
        self.skip_ws();
        let c = self.peek();
        if c == b'(' {
            self.pos += 1;
            let v = self.expr();
            self.skip_ws();
            if self.peek() != b')' {
                self.set(EvalError::Syntax);
                return 0;
            }
            self.pos += 1;
            return v;
        }
        if c == b'-' {
            self.pos += 1;
            return -self.primary();
        }
        if c.is_ascii_digit() {
            let mut v: i64 = 0;
            while self.peek().is_ascii_digit() {
                v = v * 10 + (self.peek() - b'0') as i64;
                self.pos += 1;
            }
            return v;
        }
        self.set(EvalError::Syntax);
        0
    }
    fn term(&mut self) -> i64 {
        let mut v = self.primary();
        loop {
            self.skip_ws();
            let c = self.peek();
            if c != b'*' && c != b'/' && c != b'%' {
                return v;
            }
            self.pos += 1;
            let r = self.primary();
            if c == b'*' {
                v *= r;
            } else if r == 0 {
                self.set(EvalError::DivByZero);
                return 0;
            } else if c == b'/' {
                v /= r;
            } else {
                v %= r;
            }
        }
    }
    fn expr(&mut self) -> i64 {
        let mut v = self.term();
        loop {
            self.skip_ws();
            let c = self.peek();
            if c != b'+' && c != b'-' {
                return v;
            }
            self.pos += 1;
            let r = self.term();
            v = if c == b'+' { v + r } else { v - r };
        }
    }
}

pub fn eval(s: &str) -> Result<i64, (EvalError, usize)> {
    let mut p = Parser { s: s.as_bytes(), pos: 0, err: None };
    let v = p.expr();
    p.skip_ws();
    if p.err.is_none() && p.peek() != 0 {
        p.err = Some(EvalError::Syntax);
    }
    match p.err {
        Some(e) => Err((e, p.pos)),
        None => Ok(v),
    }
}
