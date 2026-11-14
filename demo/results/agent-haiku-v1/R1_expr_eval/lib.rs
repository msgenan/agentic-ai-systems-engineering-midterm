#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvalError {
    Syntax,
    DivByZero,
}

/// Internal error state; the first error wins, as in the C code.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Fault {
    None,
    Syntax,
    DivByZero,
}

struct Parser<'a> {
    /// Input bytes, truncated at the first NUL to mirror C string semantics.
    s: &'a [u8],
    pos: usize,
    fault: Fault,
}

impl Parser<'_> {
    /// Byte at `i`, or 0 (the C terminator) past the end.
    fn at(&self, i: usize) -> u8 {
        self.s.get(i).copied().unwrap_or(0)
    }

    fn set_fault(&mut self, f: Fault) {
        if self.fault == Fault::None {
            self.fault = f;
        }
    }

    fn skip_ws(&mut self) {
        // C isspace in the "C" locale.
        while matches!(self.at(self.pos), b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r') {
            self.pos += 1;
        }
    }

    fn parse_expr(&mut self) -> i64 {
        let mut v = self.parse_term();
        loop {
            self.skip_ws();
            let c = self.at(self.pos);
            if c != b'+' && c != b'-' {
                return v;
            }
            self.pos += 1;
            let r = self.parse_term();
            v = if c == b'+' { v.wrapping_add(r) } else { v.wrapping_sub(r) };
        }
    }

    fn parse_term(&mut self) -> i64 {
        let mut v = self.parse_primary();
        loop {
            self.skip_ws();
            let c = self.at(self.pos);
            if c != b'*' && c != b'/' && c != b'%' {
                return v;
            }
            self.pos += 1;
            let r = self.parse_primary();
            if c == b'*' {
                v = v.wrapping_mul(r);
            } else if r == 0 {
                self.set_fault(Fault::DivByZero);
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
        let c = self.at(self.pos);
        if c == b'(' {
            self.pos += 1;
            let v = self.parse_expr();
            self.skip_ws();
            if self.at(self.pos) != b')' {
                self.set_fault(Fault::Syntax);
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
            while self.at(self.pos).is_ascii_digit() {
                v = v
                    .wrapping_mul(10)
                    .wrapping_add(i64::from(self.at(self.pos) - b'0'));
                self.pos += 1;
            }
            return v;
        }
        self.set_fault(Fault::Syntax);
        0
    }
}

/// Evaluates an integer expression with + - * / % ( ) and unary minus.
/// Returns Ok(value), or Err((error, offset where parsing stopped)) with the same
/// error code and offset as the C version.
pub fn eval(s: &str) -> Result<i64, (EvalError, usize)> {
    let bytes = s.as_bytes();
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let mut p = Parser {
        s: &bytes[..end],
        pos: 0,
        fault: Fault::None,
    };
    let v = p.parse_expr();
    p.skip_ws();
    if p.fault == Fault::None && p.at(p.pos) != 0 {
        p.fault = Fault::Syntax;
    }
    match p.fault {
        Fault::None => Ok(v),
        Fault::Syntax => Err((EvalError::Syntax, p.pos)),
        Fault::DivByZero => Err((EvalError::DivByZero, p.pos)),
    }
}
