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

fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r')
}

fn is_redir(t: TokenType) -> bool {
    matches!(
        t,
        TokenType::RedirIn | TokenType::RedirOut | TokenType::Heredoc | TokenType::Append
    )
}

fn push_token(tokens: &mut Vec<Token>, ty: TokenType, value: &str, quoted: i32) {
    if value.is_empty() {
        return;
    }
    tokens.push(Token {
        ty,
        value: value.to_string(),
        quoted,
    });
}

/// C `lexer()`. Returns `None` where the C function returns NULL (unclosed quote, or no tokens at all).
pub fn lexer(input: &str) -> Option<Vec<Token>> {
    let chars: Vec<char> = input.chars().collect();
    let at = |i: usize| chars.get(i).copied().unwrap_or('\0');
    let mut i = 0;
    let mut tokens = Vec::new();
    while i < chars.len() {
        while i < chars.len() && is_space(chars[i]) {
            i += 1;
        }
        let c = at(i);
        if c == '|' {
            push_token(&mut tokens, TokenType::Pipe, "|", 0);
            i += 1;
        } else if c == '>' || c == '<' {
            if c == '>' && at(i + 1) == '>' {
                push_token(&mut tokens, TokenType::Append, ">>", 0);
                i += 2;
            } else if c == '>' {
                push_token(&mut tokens, TokenType::RedirOut, ">", 0);
                i += 1;
            } else if at(i + 1) == '<' {
                push_token(&mut tokens, TokenType::Heredoc, "<<", 0);
                i += 2;
            } else {
                push_token(&mut tokens, TokenType::RedirIn, "<", 0);
                i += 1;
            }
        } else {
            let mut word = String::new();
            let mut quote_type = 0;
            while i < chars.len() {
                let c = chars[i];
                if is_space(c) || c == '|' || c == '<' || c == '>' {
                    break;
                }
                if c == '"' || c == '\'' {
                    if c == '\'' {
                        quote_type = 1;
                    }
                    word.push(if c == '\'' { '\x01' } else { '\x02' });
                    i += 1;
                    while i < chars.len() && chars[i] != c {
                        word.push(chars[i]);
                        i += 1;
                    }
                    if i >= chars.len() {
                        return None;
                    }
                    word.push('\x03');
                    i += 1;
                } else {
                    word.push(c);
                    i += 1;
                }
            }
            push_token(&mut tokens, TokenType::Word, &word, quote_type);
        }
    }
    if tokens.is_empty() {
        None
    } else {
        Some(tokens)
    }
}

fn is_invalid_char(input: &str) -> bool {
    let mut quote: Option<char> = None;
    for c in input.chars() {
        if quote.is_none() && (c == '"' || c == '\'') {
            quote = Some(c);
        } else if quote == Some(c) {
            quote = None;
        } else if quote.is_none() && (c == ';' || c == '\\') {
            return true;
        }
    }
    false
}

fn is_unclosed_quotes(input: &str) -> bool {
    let mut quote: Option<char> = None;
    for c in input.chars() {
        if quote.is_none() && (c == '"' || c == '\'') {
            quote = Some(c);
        } else if quote == Some(c) {
            quote = None;
        }
    }
    quote.is_some()
}

/// C `validate_syntax()`; `shell.input` is the line that was lexed. Returns SYNTAX_OK or an ERR_* code.
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
    for w in tokens.windows(2) {
        if is_redir(w[0].ty) && w[1].ty == TokenType::Pipe {
            return ERR_REDIR_EOF;
        }
        if w[0].ty == TokenType::Pipe && w[1].ty == TokenType::Pipe {
            return ERR_PIPE_DOUBLE;
        }
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
    let mut cmds = Vec::new();
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
                // C dereferences next unconditionally; the syntax check guarantees it exists.
                if let Some(next) = tokens.get(i + 1) {
                    cur.redir.push(Redir {
                        ty,
                        file: next.value.clone(),
                    });
                }
                i += 1;
            }
        }
        i += 1;
    }
    cmds.push(cur);
    cmds
}

fn get_env_value<'a>(env: &'a [String], key: &str) -> Option<&'a str> {
    for line in env {
        match line.find('=') {
            Some(p) => {
                if &line[..p] == key {
                    return Some(&line[p + 1..]);
                }
            }
            None => {
                if line == key {
                    return None;
                }
            }
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
    if !(c.is_ascii_alphabetic() || c == '_') {
        return "$".to_string();
    }
    let start = *i;
    while *i < chars.len() && (chars[*i].is_ascii_alphanumeric() || chars[*i] == '_') {
        *i += 1;
    }
    let name: String = chars[start..*i].iter().collect();
    get_env_value(&shell.env, &name).unwrap_or("").to_string()
}

fn expand_string(shell: &Shell, s: &str) -> String {
    if s == "''" || s == "\"\"" {
        return String::new();
    }
    if s.starts_with('\x01') && !s.contains('\x02') && !s.contains('\x03') {
        return s[1..].to_string();
    }
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut in_single = false;
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '\x01' => {
                in_single = true;
                i += 1;
            }
            '\x02' | '\x03' => {
                in_single = false;
                i += 1;
            }
            '$' if !in_single => result.push_str(&expand_var(shell, &chars, &mut i)),
            c => {
                result.push(c);
                i += 1;
            }
        }
    }
    result
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
