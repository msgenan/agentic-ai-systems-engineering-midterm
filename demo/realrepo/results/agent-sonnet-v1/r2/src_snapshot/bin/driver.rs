//! Driver: one input per line on stdin -> canonical dump on stdout. Mirrors the C driver exactly.
//! (Provided file, do not edit.)
use migrated::*;
use std::io::{BufRead, Write};

fn type_name(t: TokenType) -> &'static str {
    match t {
        TokenType::Word => "WORD",
        TokenType::Pipe => "PIPE",
        TokenType::RedirIn => "REDIR_IN",
        TokenType::RedirOut => "REDIR_OUT",
        TokenType::Heredoc => "HEREDOC",
        TokenType::Append => "APPEND",
    }
}

fn esc(s: &str) -> String {
    let mut o: Vec<u8> = vec![b'"'];
    for &c in s.as_bytes() {
        if c == b'"' || c == b'\\' {
            o.push(b'\\');
            o.push(c);
        } else if c < 0x20 || c == 0x7f {
            o.extend_from_slice(format!("\\x{:02x}", c).as_bytes());
        } else {
            o.push(c);
        }
    }
    o.push(b'"');
    String::from_utf8_lossy(&o).into_owned()
}

const ENV: [&str; 11] = [
    "HOME=/home/u", "USER=alice", "EMPTY=", "PATH=/bin:/usr/bin", "A=1", "B=hello world",
    "_UND=x", "x1=v1", "EQ=a=b", "NOVAL", "A=dup",
];

/// Same as process() in minishell main.c minus the executor.
pub fn dump(input: &str) -> String {
    let shell = Shell {
        input: input.to_string(),
        env: ENV.iter().map(|s| s.to_string()).collect(),
        exit_status: 7,
    };
    let toks = lexer(&shell.input);
    let tslice: &[Token] = toks.as_deref().unwrap_or(&[]);
    let err = validate_syntax(&shell, tslice);
    if err != SYNTAX_OK {
        return format!("ERR {}\n", err);
    }
    let mut out = String::new();
    for t in tslice {
        out.push_str(&format!("TOK {} q={} {}\n", type_name(t.ty), t.quoted, esc(&t.value)));
    }
    let mut cmds = parser(tslice);
    expand_all(&shell, &mut cmds);
    for (i, c) in cmds.iter().enumerate() {
        let argv: Vec<String> = c.argv.iter().map(|a| esc(a)).collect();
        let redir: Vec<String> =
            c.redir.iter().map(|r| format!("{} {}", type_name(r.ty), esc(&r.file))).collect();
        out.push_str(&format!("CMD {} argv=[{}] redir=[{}]\n", i, argv.join(","), redir.join(",")));
    }
    out.push_str("END\n");
    out
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut line = String::new();
    while stdin.lock().read_line(&mut line).unwrap() > 0 {
        if line.ends_with('\n') {
            line.pop();
        }
        stdout.write_all(dump(&line).as_bytes()).unwrap();
        line.clear();
    }
}
