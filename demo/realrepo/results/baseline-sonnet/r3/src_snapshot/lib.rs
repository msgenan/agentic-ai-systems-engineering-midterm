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

fn is_redir(t: TokenType) -> bool {
    matches!(
        t,
        TokenType::RedirIn | TokenType::RedirOut | TokenType::Heredoc | TokenType::Append
    )
}

fn lossy(b: Vec<u8>) -> String {
    String::from_utf8_lossy(&b).into_owned()
}

/// C `lexer()`. Returns `None` where the C function returns NULL (unclosed quote, or no tokens at all).
pub fn lexer(input: &str) -> Option<Vec<Token>> {
    let b = input.as_bytes();
    let n = b.len();
    let mut i = 0;
    let mut toks: Vec<Token> = Vec::new();
    let simple = |ty, s: &str| Token { ty, value: s.to_string(), quoted: 0 };
    while i < n {
        while i < n && is_space(b[i]) {
            i += 1;
        }
        if i >= n {
            break;
        }
        let c = b[i];
        if c == b'|' {
            toks.push(simple(TokenType::Pipe, "|"));
            i += 1;
        } else if c == b'>' || c == b'<' {
            let next = if i + 1 < n { b[i + 1] } else { 0 };
            if c == b'>' && next == b'>' {
                toks.push(simple(TokenType::Append, ">>"));
                i += 2;
            } else if c == b'>' {
                toks.push(simple(TokenType::RedirOut, ">"));
                i += 1;
            } else if next == b'<' {
                toks.push(simple(TokenType::Heredoc, "<<"));
                i += 2;
            } else {
                toks.push(simple(TokenType::RedirIn, "<"));
                i += 1;
            }
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
                toks.push(Token { ty: TokenType::Word, value: lossy(word), quoted });
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
    let mut qc = 0u8;
    for &c in input {
        if (c == b'"' || c == b'\'') && !in_quote {
            in_quote = true;
            qc = c;
        } else if c == qc && in_quote {
            in_quote = false;
            qc = 0;
        } else if !in_quote && (c == b';' || c == b'\\') {
            return true;
        }
    }
    false
}

fn is_unclosed_quotes(input: &[u8]) -> bool {
    let mut q = 0u8;
    for &c in input {
        if (c == b'"' || c == b'\'') && q == 0 {
            q = c;
        } else if c == q {
            q = 0;
        }
    }
    q != 0
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
    let mut i = 0;
    while i + 1 < tokens.len() {
        let t = &tokens[i];
        let nx = &tokens[i + 1];
        if is_redir(t.ty) && nx.ty == TokenType::Pipe {
            return ERR_REDIR_EOF;
        }
        if t.ty == TokenType::Pipe && nx.ty == TokenType::Pipe {
            return ERR_PIPE_DOUBLE;
        }
        i += 1;
    }
    if let Some(t) = tokens.last() {
        if is_redir(t.ty) {
            return ERR_REDIR_EOF;
        }
        if t.ty == TokenType::Pipe {
            return ERR_PIPE_EOF;
        }
    }
    SYNTAX_OK
}

/// C `parser()`: tokens -> commands (one per pipeline segment).
pub fn parser(tokens: &[Token]) -> Vec<Cmd> {
    let mut cmds: Vec<Cmd> = Vec::new();
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

fn get_env_value(shell: &Shell, key: &[u8]) -> Vec<u8> {
    for line in &shell.env {
        let lb = line.as_bytes();
        let eq = lb.iter().position(|&c| c == b'=');
        let k = match eq {
            Some(p) => &lb[..p],
            None => lb,
        };
        if k == key {
            return match eq {
                Some(p) => lb[p + 1..].to_vec(),
                None => Vec::new(),
            };
        }
    }
    Vec::new()
}

fn expand_string(shell: &Shell, s: &str) -> String {
    let b = s.as_bytes();
    if b == b"''" || b == b"\"\"" {
        return String::new();
    }
    if !b.is_empty() && b[0] == 1 && !b.contains(&2) && !b.contains(&3) {
        return lossy(b[1..].to_vec());
    }
    let n = b.len();
    let mut out: Vec<u8> = Vec::new();
    let mut i = 0;
    let mut single = false;
    while i < n {
        let c = b[i];
        if c == 1 {
            single = true;
            i += 1;
        } else if c == 2 || c == 3 {
            single = false;
            i += 1;
        } else if c == b'$' && !single {
            i += 1;
            let cur = if i < n { b[i] } else { 0 };
            if cur == b'?' {
                i += 1;
                out.extend_from_slice(shell.exit_status.to_string().as_bytes());
            } else if cur == b'$' {
                i += 1;
            } else if !(cur.is_ascii_alphabetic() || cur == b'_') {
                out.push(b'$');
            } else {
                let start = i;
                while i < n && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                    i += 1;
                }
                out.extend_from_slice(&get_env_value(shell, &b[start..i]));
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    lossy(out)
}

/// C `expand_all()`: expands args and redirection targets of every command in place.
pub fn expand_all(shell: &Shell, cmds: &mut [Cmd]) {
    for cmd in cmds.iter_mut() {
        for i in 0..cmd.argv.len() {
            if i == 0 || cmd.argv[0] != "export" {
                cmd.argv[i] = expand_string(shell, &cmd.argv[i]);
            } else if cmd.argv[i].contains('=') {
                cmd.argv[i] = expand_string(shell, &cmd.argv[i]);
            }
        }
        for r in cmd.redir.iter_mut() {
            r.file = expand_string(shell, &r.file);
        }
    }
}
