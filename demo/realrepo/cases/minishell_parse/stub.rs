//! Rust port of the minishell lexer + syntax check + parser + variable/quote expansion path.
//! Replace every `todo!()` with a faithful translation of the C code in `c_src/`.
//! Keep the public API (names, signatures) unchanged: `src/bin/driver.rs` and the tests use it.

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

/// C `lexer()`. Returns `None` where the C function returns NULL (unclosed quote, or no tokens at all).
pub fn lexer(_input: &str) -> Option<Vec<Token>> {
    todo!()
}

/// C `validate_syntax()`; `shell.input` is the line that was lexed. Returns SYNTAX_OK or an ERR_* code.
pub fn validate_syntax(_shell: &Shell, _tokens: &[Token]) -> i32 {
    todo!()
}

/// C `parser()`: tokens -> commands (one per pipeline segment).
pub fn parser(_tokens: &[Token]) -> Vec<Cmd> {
    todo!()
}

/// C `expand_all()`: expands args and redirection targets of every command in place.
pub fn expand_all(_shell: &Shell, _cmds: &mut [Cmd]) {
    todo!()
}
