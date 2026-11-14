//! Visible tests: expected output was produced by the C driver (cdriver/driver.c).
use std::io::Write;
use std::process::{Command, Stdio};

fn run(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_driver"))
        .stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(format!("{}\n", input).as_bytes()).unwrap();
    String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap()
}

#[test]
fn case_01() {
    assert_eq!(run("echo hello world"), "TOK WORD q=0 \"echo\"\nTOK WORD q=0 \"hello\"\nTOK WORD q=0 \"world\"\nCMD 0 argv=[\"echo\",\"hello\",\"world\"] redir=[]\nEND\n");
}

#[test]
fn case_02() {
    assert_eq!(run("echo \"hi $USER\" 'a$B' $HOME"), "TOK WORD q=0 \"echo\"\nTOK WORD q=0 \"\\x02hi $USER\\x03\"\nTOK WORD q=1 \"\\x01a$B\\x03\"\nTOK WORD q=0 \"$HOME\"\nCMD 0 argv=[\"echo\",\"hi alice\",\"a$B\",\"/home/u\"] redir=[]\nEND\n");
}

#[test]
fn case_03() {
    assert_eq!(run("cat < in > out"), "TOK WORD q=0 \"cat\"\nTOK REDIR_IN q=0 \"<\"\nTOK WORD q=0 \"in\"\nTOK REDIR_OUT q=0 \">\"\nTOK WORD q=0 \"out\"\nCMD 0 argv=[\"cat\"] redir=[REDIR_IN \"in\",REDIR_OUT \"out\"]\nEND\n");
}

#[test]
fn case_04() {
    assert_eq!(run("a | b | c"), "TOK WORD q=0 \"a\"\nTOK PIPE q=0 \"|\"\nTOK WORD q=0 \"b\"\nTOK PIPE q=0 \"|\"\nTOK WORD q=0 \"c\"\nCMD 0 argv=[\"a\"] redir=[]\nCMD 1 argv=[\"b\"] redir=[]\nCMD 2 argv=[\"c\"] redir=[]\nEND\n");
}

#[test]
fn case_05() {
    assert_eq!(run("echo \"unclosed"), "ERR 6\n");
}

#[test]
fn case_06() {
    assert_eq!(run("| a"), "ERR 2\n");
}

#[test]
fn case_07() {
    assert_eq!(run("cmd >"), "ERR 4\n");
}

#[test]
fn case_08() {
    assert_eq!(run("echo $NOSUCH done"), "TOK WORD q=0 \"echo\"\nTOK WORD q=0 \"$NOSUCH\"\nTOK WORD q=0 \"done\"\nCMD 0 argv=[\"echo\",\"\",\"done\"] redir=[]\nEND\n");
}

#[test]
fn case_09() {
    assert_eq!(run("echo $?"), "TOK WORD q=0 \"echo\"\nTOK WORD q=0 \"$?\"\nCMD 0 argv=[\"echo\",\"7\"] redir=[]\nEND\n");
}

#[test]
fn case_10() {
    assert_eq!(run("echo a;b"), "ERR 1\n");
}

#[test]
fn case_11() {
    assert_eq!(run("cat <<EOF >> f"), "TOK WORD q=0 \"cat\"\nTOK HEREDOC q=0 \"<<\"\nTOK WORD q=0 \"EOF\"\nTOK APPEND q=0 \">>\"\nTOK WORD q=0 \"f\"\nCMD 0 argv=[\"cat\"] redir=[HEREDOC \"EOF\",APPEND \"f\"]\nEND\n");
}

#[test]
fn case_12() {
    assert_eq!(run("   "), "CMD 0 argv=[] redir=[]\nEND\n");
}
