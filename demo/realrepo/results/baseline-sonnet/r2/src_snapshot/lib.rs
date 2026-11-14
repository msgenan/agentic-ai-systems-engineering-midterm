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

fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r')
}

fn is_redir(t: TokenType) -> bool {
    matches!(
        t,
        TokenType::RedirIn | TokenType::RedirOut | TokenType::Append | TokenType::Heredoc
    )
}

/// C `lexer()`. Returns `None` where the C function returns NULL (unclosed quote, or no tokens at all).
pub fn lexer(input: &str) -> Option<Vec<Token>> {
    let chars: Vec<char> = input.chars().collect();
    let n = chars.len();
    let mut i = 0;
    let mut tokens: Vec<Token> = Vec::new();
    while i < n {
        while i < n && is_space(chars[i]) {
            i += 1;
        }
        if i >= n {
            break;
        }
        let c = chars[i];
        let simple = |ty: TokenType, s: &str| Token { ty, value: s.to_string(), quoted: 0 };
        if c == '|' {
            tokens.push(simple(TokenType::Pipe, "|"));
            i += 1;
        } else if c == '>' || c == '<' {
            let next = if i + 1 < n { chars[i + 1] } else { '\0' };
            if c == '>' && next == '>' {
                tokens.push(simple(TokenType::Append, ">>"));
                i += 2;
            } else if c == '>' {
                tokens.push(simple(TokenType::RedirOut, ">"));
                i += 1;
            } else if c == '<' && next == '<' {
                tokens.push(simple(TokenType::Heredoc, "<<"));
                i += 2;
            } else {
                tokens.push(simple(TokenType::RedirIn, "<"));
                i += 1;
            }
        } else {
            let mut result = String::new();
            let mut quote_type = 0;
            while i < n && !is_space(chars[i]) && !matches!(chars[i], '|' | '<' | '>') {
                let ch = chars[i];
                if ch == '"' || ch == '\'' {
                    if ch == '\'' {
                        quote_type = 1;
                    }
                    result.push(if ch == '\'' { '\x01' } else { '\x02' });
                    i += 1;
                    while i < n && chars[i] != ch {
                        result.push(chars[i]);
                        i += 1;
                    }
                    if i >= n {
                        return None;
                    }
                    result.push('\x03');
                    i += 1;
                } else {
                    result.push(ch);
                    i += 1;
                }
            }
            if !result.is_empty() {
                tokens.push(Token { ty: TokenType::Word, value: result, quoted: quote_type });
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
    let mut quote_char = '\0';
    for c in input.chars() {
        if (c == '"' || c == '\'') && !in_quote {
            in_quote = true;
            quote_char = c;
        } else if c == quote_char && in_quote {
            in_quote = false;
            quote_char = '\0';
        } else if !in_quote && (c == ';' || c == '\\') {
            return true;
        }
    }
    false
}

fn is_unclosed_quotes(input: &str) -> bool {
    let mut quote = '\0';
    for c in input.chars() {
        if (c == '"' || c == '\'') && quote == '\0' {
            quote = c;
        } else if c == quote {
            quote = '\0';
        }
    }
    quote != '\0'
}

/// C `validate_syntax()`.
pub fn validate_syntax(shell: &Shell, tokens: &[Token]) -> i32 {
    if is_invalid_char(&shell.input) {
        return ERR_INVALID;
    }
    if is_unclosed_quotes(&shell.input) {
        return ERR_UNCLOSED_QUOTES;
    }
    if let Some(t) = tokens.first() {
        if t.ty == TokenType::Pipe {
            return ERR_PIPE_START;
        }
    }
    if tokens.len() > 1 {
        for i in 0..tokens.len() - 1 {
            let t = &tokens[i];
            let next = &tokens[i + 1];
            if is_redir(t.ty) && next.ty == TokenType::Pipe {
                return ERR_REDIR_EOF;
            }
            if t.ty == TokenType::Pipe && next.ty == TokenType::Pipe {
                return ERR_PIPE_DOUBLE;
            }
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
    let mut cmds: Vec<Cmd> = Vec::new();
    let mut cur = Cmd::default();
    let mut i = 0;
    while i < tokens.len() {
        let t = &tokens[i];
        match t.ty {
            TokenType::Word => {
                if t.quoted == 1 {
                    cur.argv.push(format!("\x01{}", t.value));
                } else {
                    cur.argv.push(t.value.clone());
                }
            }
            TokenType::Pipe => {
                cmds.push(std::mem::take(&mut cur));
            }
            ty => {
                if let Some(next) = tokens.get(i + 1) {
                    cur.redir.push(Redir { ty, file: next.value.clone() });
                }
                i += 1;
            }
        }
        i += 1;
    }
    cmds.push(cur);
    cmds
}

fn get_env_value(env: &[String], key: &str) -> Option<String> {
    for line in env {
        let (k, v) = match line.find('=') {
            Some(p) => (&line[..p], Some(&line[p + 1..])),
            None => (&line[..], None),
        };
        if k == key {
            return v.map(|s| s.to_string());
        }
    }
    None
}

fn expand_var(shell: &Shell, chars: &[char], i: &mut usize) -> String {
    *i += 1;
    let c = chars.get(*i).copied().unwrap_or('\0');
    if c == '?' {
        *i += 1;
        return shell.exit_status.to_string();
    }
    if c == '$' {
        *i += 1;
        return String::new();
    }
    if !c.is_ascii_alphabetic() && c != '_' {
        return "$".to_string();
    }
    let start = *i;
    while *i < chars.len() && (chars[*i].is_ascii_alphanumeric() || chars[*i] == '_') {
        *i += 1;
    }
    let name: String = chars[start..*i].iter().collect();
    get_env_value(&shell.env, &name).unwrap_or_default()
}

fn expand_string(shell: &Shell, s: &str) -> String {
    if s == "''" || s == "\"\"" {
        return String::new();
    }
    if s.starts_with('\x01') && !s.contains('\x02') && !s.contains('\x03') {
        return s['\x01'.len_utf8()..].to_string();
    }
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut i = 0;
    let mut in_single = false;
    while i < chars.len() {
        let c = chars[i];
        if c == '\x01' {
            in_single = true;
            i += 1;
        } else if c == '\x02' || c == '\x03' {
            in_single = false;
            i += 1;
        } else if c == '$' && !in_single {
            result.push_str(&expand_var(shell, &chars, &mut i));
        } else {
            result.push(c);
            i += 1;
        }
    }
    result
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
