//! Faithful reference translation (used to validate the harness; it is not shown to the models).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType { Word, Pipe, RedirIn, RedirOut, Heredoc, Append }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token { pub ty: TokenType, pub value: String, pub quoted: i32 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redir { pub ty: TokenType, pub file: String }

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cmd { pub argv: Vec<String>, pub redir: Vec<Redir> }

#[derive(Debug, Clone)]
pub struct Shell { pub input: String, pub env: Vec<String>, pub exit_status: i32 }

pub const SYNTAX_OK: i32 = 0;
pub const ERR_INVALID: i32 = 1;
pub const ERR_PIPE_START: i32 = 2;
pub const ERR_PIPE_DOUBLE: i32 = 3;
pub const ERR_REDIR_EOF: i32 = 4;
pub const ERR_PIPE_EOF: i32 = 5;
pub const ERR_UNCLOSED_QUOTES: i32 = 6;

fn is_space(c: char) -> bool { matches!(c, ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r') }

pub fn lexer(input: &str) -> Option<Vec<Token>> {
    let cs: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut toks = Vec::new();
    while i < cs.len() {
        while i < cs.len() && is_space(cs[i]) { i += 1; }
        if i >= cs.len() { break; }
        let c = cs[i];
        if c == '|' {
            toks.push(Token { ty: TokenType::Pipe, value: "|".into(), quoted: 0 });
            i += 1;
        } else if c == '>' || c == '<' {
            let dbl = i + 1 < cs.len() && cs[i + 1] == c;
            let (ty, v) = match (c, dbl) {
                ('>', true) => (TokenType::Append, ">>"),
                ('>', false) => (TokenType::RedirOut, ">"),
                ('<', true) => (TokenType::Heredoc, "<<"),
                _ => (TokenType::RedirIn, "<"),
            };
            toks.push(Token { ty, value: v.into(), quoted: 0 });
            i += if dbl { 2 } else { 1 };
        } else {
            let mut word = String::new();
            let mut q = 0;
            while i < cs.len() && !is_space(cs[i]) && !matches!(cs[i], '|' | '<' | '>') {
                let c = cs[i];
                if c == '"' || c == '\'' {
                    if c == '\'' { q = 1; }
                    word.push(if c == '\'' { '\x01' } else { '\x02' });
                    i += 1;
                    while i < cs.len() && cs[i] != c { word.push(cs[i]); i += 1; }
                    if i >= cs.len() { return None; }
                    word.push('\x03');
                    i += 1;
                } else {
                    word.push(c);
                    i += 1;
                }
            }
            if !word.is_empty() { toks.push(Token { ty: TokenType::Word, value: word, quoted: q }); }
        }
    }
    if toks.is_empty() { None } else { Some(toks) }
}

fn is_redir(t: TokenType) -> bool {
    matches!(t, TokenType::RedirIn | TokenType::RedirOut | TokenType::Append | TokenType::Heredoc)
}

pub fn validate_syntax(shell: &Shell, tokens: &[Token]) -> i32 {
    // is_invalid_char
    let (mut inq, mut qc) = (false, '\0');
    for c in shell.input.chars() {
        if (c == '"' || c == '\'') && !inq { inq = true; qc = c; }
        else if c == qc && inq { inq = false; qc = '\0'; }
        else if !inq && (c == ';' || c == '\\') { return ERR_INVALID; }
    }
    // is_unclosed_quotes
    let mut q = '\0';
    for c in shell.input.chars() {
        if (c == '"' || c == '\'') && q == '\0' { q = c; } else if c == q { q = '\0'; }
    }
    if q != '\0' { return ERR_UNCLOSED_QUOTES; }
    if let Some(t) = tokens.first() { if t.ty == TokenType::Pipe { return ERR_PIPE_START; } }
    for i in 0..tokens.len().saturating_sub(1) {
        let (t, n) = (&tokens[i], &tokens[i + 1]);
        if is_redir(t.ty) && n.ty == TokenType::Pipe { return ERR_REDIR_EOF; }
        if t.ty == TokenType::Pipe && n.ty == TokenType::Pipe { return ERR_PIPE_DOUBLE; }
    }
    if let Some(l) = tokens.last() {
        if is_redir(l.ty) { return ERR_REDIR_EOF; }
        if l.ty == TokenType::Pipe { return ERR_PIPE_EOF; }
    }
    SYNTAX_OK
}

pub fn parser(tokens: &[Token]) -> Vec<Cmd> {
    let mut cmds = Vec::new();
    let mut cur = Cmd::default();
    let mut i = 0;
    while i < tokens.len() {
        let t = &tokens[i];
        if t.ty == TokenType::Word {
            if t.quoted == 1 { cur.argv.push(format!("\x01{}", t.value)); } else { cur.argv.push(t.value.clone()); }
        } else if is_redir(t.ty) {
            if let Some(n) = tokens.get(i + 1) { cur.redir.push(Redir { ty: t.ty, file: n.value.clone() }); }
            i += 1;
        } else {
            cmds.push(std::mem::take(&mut cur));
        }
        i += 1;
    }
    cmds.push(cur);
    cmds
}

fn env_get(env: &[String], key: &str) -> Option<String> {
    for line in env {
        let (k, v) = match line.find('=') { Some(p) => (&line[..p], Some(&line[p + 1..])), None => (&line[..], None) };
        if k == key { return v.map(|s| s.to_string()); }
    }
    None
}

fn expand_string(shell: &Shell, s: &str) -> String {
    if s == "''" || s == "\"\"" { return String::new(); }
    if s.starts_with('\x01') && !s.contains('\x02') && !s.contains('\x03') { return s[1..].to_string(); }
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut single = false;
    let mut out = String::new();
    while i < cs.len() {
        match cs[i] {
            '\x01' => { single = true; i += 1; }
            '\x02' | '\x03' => { single = false; i += 1; }
            '$' if !single => {
                i += 1;
                if i < cs.len() && cs[i] == '?' { out.push_str(&shell.exit_status.to_string()); i += 1; }
                else if i < cs.len() && cs[i] == '$' { i += 1; }
                else if i >= cs.len() || !(cs[i].is_ascii_alphabetic() || cs[i] == '_') { out.push('$'); }
                else {
                    let st = i;
                    while i < cs.len() && (cs[i].is_ascii_alphanumeric() || cs[i] == '_') { i += 1; }
                    let name: String = cs[st..i].iter().collect();
                    if let Some(v) = env_get(&shell.env, &name) { out.push_str(&v); }
                }
            }
            c => { out.push(c); i += 1; }
        }
    }
    out
}

pub fn expand_all(shell: &Shell, cmds: &mut [Cmd]) {
    for cmd in cmds.iter_mut() {
        if !cmd.argv.is_empty() {
            for i in 0..cmd.argv.len() {
                if i == 0 || cmd.argv[0] != "export" || cmd.argv[i].contains('=') {
                    cmd.argv[i] = expand_string(shell, &cmd.argv[i]);
                }
            }
        }
        for r in cmd.redir.iter_mut() { r.file = expand_string(shell, &r.file); }
    }
}
