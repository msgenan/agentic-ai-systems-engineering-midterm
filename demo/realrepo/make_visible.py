"""Generate cases/minishell_parse/visible.rs: expected dumps come from running the C driver (never hand-written)."""
import subprocess, sys
from pathlib import Path

HERE = Path(__file__).parent
CASE = HERE / "cases" / "minishell_parse"
INPUTS = [
    "echo hello world", 'echo "hi $USER" \'a$B\' $HOME', "cat < in > out", "a | b | c", 'echo "unclosed', "| a",
    "cmd >", "echo $NOSUCH done", "echo $?", "echo a;b", "cat <<EOF >> f", "   ",
]


def rs(t: str) -> str:
    o = '"'
    for ch in t:
        if ch in '"\\': o += "\\" + ch
        elif ch == "\n": o += "\\n"
        elif ord(ch) < 0x20 or ord(ch) == 0x7F: o += f"\\x{ord(ch):02x}"
        else: o += ch
    return o + '"'


def main(c_bin: str) -> None:
    items = []
    for s in INPUTS:
        out = subprocess.run([c_bin], input=s + "\n", capture_output=True, text=True, check=True).stdout
        items.append((s, out))
    body = ['//! Visible tests: expected output was produced by the C driver (cdriver/driver.c).', "use std::io::Write;",
            "use std::process::{Command, Stdio};", "",
            "fn run(input: &str) -> String {",
            '    let mut child = Command::new(env!("CARGO_BIN_EXE_driver"))',
            "        .stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();",
            '    child.stdin.take().unwrap().write_all(format!("{}\\n", input).as_bytes()).unwrap();',
            "    String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap()", "}", ""]
    for i, (s, out) in enumerate(items, 1):
        body += ["#[test]", f"fn case_{i:02}() {{",
                 f"    assert_eq!(run({rs(s)}), {rs(out)});", "}", ""]
    (CASE / "visible.rs").write_text("\n".join(body))


main(sys.argv[1])
