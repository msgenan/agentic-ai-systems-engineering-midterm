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

/// The parts of C `t_shell` that this path uses. `env` holds raw `KEY=VALUE` (or bare `KEY`) lines, exactly
/// like the `envp` given to C `init_env`. `exit_status` is the value `$?` expands to.
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

fn to_string(b: Vec<u8>) -> String {
    match String::from_utf8(b) {
        Ok(s) => s,
        Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    }
}

/// C `lexer()`. Returns `None` where the C function returns NULL (unclosed quote, or no tokens at all).
pub fn lexer(input: &str) -> Option<Vec<Token>> {
    let b = input.as_bytes();
    let mut i = 0;
    let mut tokens: Vec<Token> = Vec::new();
    let simple = |ty, v: &str| Token { ty, value: v.to_string(), quoted: 0 };
    while i < b.len() {
        while i < b.len() && is_space(b[i]) {
            i += 1;
        }
        let c = b.get(i).copied().unwrap_or(0);
        if c == b'|' {
            tokens.push(simple(TokenType::Pipe, "|"));
            i += 1;
        } else if c == b'>' || c == b'<' {
            let n = b.get(i + 1).copied().unwrap_or(0);
            if c == b'>' && n == b'>' {
                tokens.push(simple(TokenType::Append, ">>"));
                i += 2;
            } else if c == b'>' {
                tokens.push(simple(TokenType::RedirOut, ">"));
                i += 1;
            } else if n == b'<' {
                tokens.push(simple(TokenType::Heredoc, "<<"));
                i += 2;
            } else {
                tokens.push(simple(TokenType::RedirIn, "<"));
                i += 1;
            }
        } else {
            let mut word: Vec<u8> = Vec::new();
            let mut quoted = 0;
            while i < b.len() && !is_space(b[i]) && !matches!(b[i], b'|' | b'<' | b'>') {
                let ch = b[i];
                if ch == b'"' || ch == b'\'' {
                    if ch == b'\'' {
                        quoted = 1;
                    }
                    word.push(if ch == b'\'' { 1 } else { 2 });
                    i += 1;
                    while i < b.len() && b[i] != ch {
                        word.push(b[i]);
                        i += 1;
                    }
                    if i >= b.len() {
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
                tokens.push(Token { ty: TokenType::Word, value: to_string(word), quoted });
            }
        }
    }
    if tokens.is_empty() {
        None
    } else {
        Some(tokens)
    }
}

fn is_invalid_char(input: &str) -> bool {
    let mut in_quote = false;
    let mut quote_char = 0u8;
    for &c in input.as_bytes() {
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

fn is_unclosed_quotes(input: &str) -> bool {
    let mut quote = 0u8;
    for &c in input.as_bytes() {
        if (c == b'"' || c == b'\'') && quote == 0 {
            quote = c;
        } else if c == quote {
            quote = 0;
        }
    }
    quote != 0
}

fn is_redir(t: TokenType) -> bool {
    matches!(
        t,
        TokenType::RedirIn | TokenType::RedirOut | TokenType::Append | TokenType::Heredoc
    )
}

/// C `validate_syntax()`; `shell.input` is the line that was lexed. Returns SYNTAX_OK or an ERR_* code.
pub fn validate_syntax(shell: &Shell, tokens: &[Token]) -> i32 {
    if is_invalid_char(&shell.input) {
        return ERR_INVALID;
    }
    if is_unclosed_quotes(&shell.input) {
        return ERR_UNCLOSED_QUOTES;
    }
    if let Some(f) = tokens.first() {
        if f.ty == TokenType::Pipe {
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
    if let Some(l) = tokens.last() {
        if is_redir(l.ty) {
            return ERR_REDIR_EOF;
        }
        if l.ty == TokenType::Pipe {
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
            TokenType::Pipe => cmds.push(std::mem::take(&mut cur)),
            ty => {
                if let Some(n) = tokens.get(i + 1) {
                    cur.redir.push(Redir { ty, file: n.value.clone() });
                }
                i += 1;
            }
        }
        i += 1;
    }
    cmds.push(cur);
    cmds
}

fn get_env_value<'a>(env: &'a [String], key: &[u8]) -> Option<&'a str> {
    for line in env {
        let lb = line.as_bytes();
        let eq = lb.iter().position(|&c| c == b'=');
        let k = &lb[..eq.unwrap_or(lb.len())];
        if k == key {
            // a bare KEY has a NULL value
            return eq.map(|e| &line[e + 1..]);
        }
    }
    None
}

fn expand_string(shell: &Shell, s: &str) -> String {
    let b = s.as_bytes();
    if b == b"''" || b == b"\"\"" {
        return String::new();
    }
    if b.first() == Some(&1) && !b.contains(&2) && !b.contains(&3) {
        return s[1..].to_string();
    }
    let mut out: Vec<u8> = Vec::new();
    let mut single = false;
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            1 => {
                single = true;
                i += 1;
            }
            2 | 3 => {
                single = false;
                i += 1;
            }
            b'$' if !single => {
                i += 1;
                let c = b.get(i).copied().unwrap_or(0);
                if c == b'?' {
                    i += 1;
                    out.extend(shell.exit_status.to_string().bytes());
                } else if c == b'$' {
                    i += 1;
                } else if !(c.is_ascii_alphabetic() || c == b'_') {
                    out.push(b'$');
                } else {
                    let st = i;
                    while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                        i += 1;
                    }
                    if let Some(v) = get_env_value(&shell.env, &b[st..i]) {
                        out.extend(v.bytes());
                    }
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    to_string(out)
}

/// C `expand_all()`: expands args and redirection targets of every command in place.
pub fn expand_all(shell: &Shell, cmds: &mut [Cmd]) {
    for cmd in cmds.iter_mut() {
        for i in 0..cmd.argv.len() {
            if i == 0 || cmd.argv[0] != "export" || cmd.argv[i].contains('=') {
                cmd.argv[i] = expand_string(shell, &cmd.argv[i]);
            }
        }
        for r in cmd.redir.iter_mut() {
            r.file = expand_string(shell, &r.file);
        }
    }
}
