//! Rust port of the minishell lexer + syntax check + parser + variable/quote expansion path.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Word,
    Pipe,
    RedirIn,
    RedirOut,
    Heredoc,
    Append,
}

/// C `t_token` (the linked list becomes a Vec). `quoted` is 1 if the word contained a single-quoted part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub ty: TokenType,
    pub value: String,
    pub quoted: i32,
}

/// C `t_redir`. `ty` is one of RedirIn / RedirOut / Heredoc / Append.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redir {
    pub ty: TokenType,
    pub file: String,
}

/// C `t_cmd`. `argv` is empty when the C `argv` pointer is NULL.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cmd {
    pub argv: Vec<String>,
    pub redir: Vec<Redir>,
}

/// The parts of C `t_shell` that this path uses.
#[derive(Debug, Clone)]
pub struct Shell {
    pub input: String,
    pub env: Vec<String>,
    pub exit_status: i32,
}

pub const SYNTAX_OK: i32 = 0;
pub const ERR_INVALID: i32 = 1;
pub const ERR_PIPE_START: i32 = 2;
pub const ERR_PIPE_DOUBLE: i32 = 3;
pub const ERR_REDIR_EOF: i32 = 4;
pub const ERR_PIPE_EOF: i32 = 5;
pub const ERR_UNCLOSED_QUOTES: i32 = 6;

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

fn to_string(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// C `lexer()`. Returns `None` where the C function returns NULL (unclosed quote, or no tokens at all).
pub fn lexer(input: &str) -> Option<Vec<Token>> {
    let b = input.as_bytes();
    let n = b.len();
    let mut i = 0;
    let mut toks: Vec<Token> = Vec::new();
    while i < n {
        while i < n && is_space(b[i]) {
            i += 1;
        }
        if i >= n {
            break;
        }
        let c = b[i];
        if c == b'|' {
            toks.push(Token { ty: TokenType::Pipe, value: "|".into(), quoted: 0 });
            i += 1;
        } else if c == b'>' || c == b'<' {
            let next = if i + 1 < n { b[i + 1] } else { 0 };
            let (ty, s, adv) = if c == b'>' && next == b'>' {
                (TokenType::Append, ">>", 2)
            } else if c == b'>' {
                (TokenType::RedirOut, ">", 1)
            } else if next == b'<' {
                (TokenType::Heredoc, "<<", 2)
            } else {
                (TokenType::RedirIn, "<", 1)
            };
            toks.push(Token { ty, value: s.into(), quoted: 0 });
            i += adv;
        } else {
            let mut word: Vec<u8> = Vec::new();
            let mut quoted = 0;
            while i < n && !is_space(b[i]) && b[i] != b'|' && b[i] != b'<' && b[i] != b'>' {
                let ch = b[i];
                if ch == b'"' || ch == b'\'' {
                    if ch == b'\'' {
                        quoted = 1;
                    }
                    word.push(if ch == b'\'' { 1 } else { 2 });
                    i += 1;
                    while i < n && b[i] != ch {
                        word.push(b[i]);
                        i += 1;
                    }
                    if i >= n {
                        return None;
                    }
                    word.push(3);
                    i += 1;
                } else {
                    word.push(ch);
                    i += 1;
                }
            }
            if !word.is_empty() {
                toks.push(Token { ty: TokenType::Word, value: to_string(&word), quoted });
            }
        }
    }
    if toks.is_empty() {
        None
    } else {
        Some(toks)
    }
}

fn is_invalid_char(input: &[u8]) -> bool {
    let mut in_quote = false;
    let mut quote_char = 0u8;
    for &c in input {
        if (c == b'"' || c == b'\'') && !in_quote {
            in_quote = true;
            quote_char = c;
        } else if c == quote_char && in_quote {
            in_quote = false;
            quote_char = 0;
        } else if !in_quote && (c == b';' || c == b'\\') {
            return true;
        }
    }
    false
}

fn is_unclosed_quotes(input: &[u8]) -> bool {
    let mut quote = 0u8;
    for &c in input {
        if (c == b'"' || c == b'\'') && quote == 0 {
            quote = c;
        } else if c == quote {
            quote = 0;
        }
    }
    quote != 0
}

fn is_redir(t: TokenType) -> bool {
    matches!(t, TokenType::RedirIn | TokenType::RedirOut | TokenType::Append | TokenType::Heredoc)
}

/// C `validate_syntax()`; `shell.input` is the line that was lexed. Returns SYNTAX_OK or an ERR_* code.
pub fn validate_syntax(shell: &Shell, tokens: &[Token]) -> i32 {
    let inp = shell.input.as_bytes();
    if is_invalid_char(inp) {
        return ERR_INVALID;
    }
    if is_unclosed_quotes(inp) {
        return ERR_UNCLOSED_QUOTES;
    }
    if let Some(t) = tokens.first() {
        if t.ty == TokenType::Pipe {
            return ERR_PIPE_START;
        }
    }
    for w in tokens.windows(2) {
        if is_redir(w[0].ty) && w[1].ty == TokenType::Pipe {
            return ERR_REDIR_EOF;
        }
        if w[0].ty == TokenType::Pipe && w[1].ty == TokenType::Pipe {
            return ERR_PIPE_DOUBLE;
        }
    }
    if let Some(last) = tokens.last() {
        if is_redir(last.ty) {
            return ERR_REDIR_EOF;
        }
        if last.ty == TokenType::Pipe {
            return ERR_PIPE_EOF;
        }
    }
    SYNTAX_OK
}

/// C `parser()`: tokens -> commands (one per pipeline segment).
pub fn parser(tokens: &[Token]) -> Vec<Cmd> {
    let mut cmds = Vec::new();
    let mut cur = Cmd::default();
    let mut i = 0;
    while i < tokens.len() {
        let t = &tokens[i];
        match t.ty {
            TokenType::Word => {
                if t.quoted == 1 {
                    cur.argv.push(format!("\u{1}{}", t.value));
                } else {
                    cur.argv.push(t.value.clone());
                }
            }
            TokenType::Pipe => {
                cmds.push(std::mem::take(&mut cur));
            }
            ty => {
                if let Some(nx) = tokens.get(i + 1) {
                    cur.redir.push(Redir { ty, file: nx.value.clone() });
                }
                i += 1;
            }
        }
        i += 1;
    }
    cmds.push(cur);
    cmds
}

fn expand_var(env: &[(Vec<u8>, Option<Vec<u8>>)], status: i32, s: &[u8], p: &mut usize) -> Vec<u8> {
    *p += 1;
    let at = |i: usize| -> u8 { s.get(i).copied().unwrap_or(0) };
    let c = at(*p);
    if c == b'?' {
        *p += 1;
        return status.to_string().into_bytes();
    }
    if c == b'$' {
        *p += 1;
        return Vec::new();
    }
    if !c.is_ascii_alphabetic() && c != b'_' {
        return b"$".to_vec();
    }
    let start = *p;
    while at(*p).is_ascii_alphanumeric() || at(*p) == b'_' {
        *p += 1;
    }
    let name = &s[start..*p];
    for (k, v) in env {
        if k.as_slice() == name {
            return v.clone().unwrap_or_default();
        }
    }
    Vec::new()
}

fn expand_string(env: &[(Vec<u8>, Option<Vec<u8>>)], status: i32, s: &str) -> String {
    let b = s.as_bytes();
    if b == b"''" || b == b"\"\"" {
        return String::new();
    }
    if b.first() == Some(&1) && !b.contains(&2) && !b.contains(&3) {
        return to_string(&b[1..]);
    }
    let mut res: Vec<u8> = Vec::new();
    let mut p = 0;
    let mut single = false;
    while p < b.len() {
        let c = b[p];
        if c == 1 {
            single = true;
            p += 1;
        } else if c == 2 || c == 3 {
            single = false;
            p += 1;
        } else if c == b'$' && !single {
            let part = expand_var(env, status, b, &mut p);
            res.extend_from_slice(&part);
        } else {
            res.push(c);
            p += 1;
        }
    }
    to_string(&res)
}

/// C `expand_all()`: expands args and redirection targets of every command in place.
pub fn expand_all(shell: &Shell, cmds: &mut [Cmd]) {
    let env: Vec<(Vec<u8>, Option<Vec<u8>>)> = shell
        .env
        .iter()
        .map(|l| {
            let b = l.as_bytes();
            match b.iter().position(|&c| c == b'=') {
                Some(i) => (b[..i].to_vec(), Some(b[i + 1..].to_vec())),
                None => (b.to_vec(), None),
            }
        })
        .collect();
    let st = shell.exit_status;
    for cmd in cmds.iter_mut() {
        for i in 0..cmd.argv.len() {
            if i == 0 || cmd.argv[0] != "export" || cmd.argv[i].contains('=') {
                let e = expand_string(&env, st, &cmd.argv[i]);
                cmd.argv[i] = e;
            }
        }
        for r in cmd.redir.iter_mut() {
            r.file = expand_string(&env, st, &r.file);
        }
    }
}
