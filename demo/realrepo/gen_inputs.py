"""Generate the hidden differential inputs (seeded) -> cases/minishell_parse/hidden_inputs.txt (one per line)."""
import random
from pathlib import Path

OUT = Path(__file__).parent / "cases" / "minishell_parse" / "hidden_inputs.txt"

HAND = [
    "", "   ", "\t", "ls", "ls -la /tmp", "echo hello world", "echo \"a  b\"", "echo 'a  b'", "echo $HOME", "echo \"$HOME\"",
    "echo '$HOME'", "echo $A", "echo $NOVAL.x", "echo $EMPTY", "echo $?", "echo \"$?\"", "echo $$", "echo $$$A", "echo $1",
    "echo $", "echo $ x", "echo \"$\"", "echo $é", "echo é$A", "echo $_UND", "echo $x1y", "echo $EQ", "echo ${A}", "echo $A$B$USER",
    "echo a$A'b'", "echo $A'b'", "echo 'a'$A", "echo \"a\"$A", "echo a'$A'$A", "echo \"$A\"'$A'\"$A\"", "echo ''", "echo \"\"",
    "echo '' \"\" x", "echo \"'\"", "echo '\"'", "echo \"it's\"", "cat < in > out", "cat <in>out", "cat >>out <<EOF", "cat > > f",
    "> f", ">> f cmd", "cmd >", "cmd >>", "cmd <", "cmd <<", "cmd < | x", "a | b | c", "a|b", "a || b", "| a", "a |", "a | | b", "a ||| b",
    "echo a;b", "echo 'a;b'", "echo \"a\\b\"", "echo a\\b", "echo \"unclosed", "echo 'unclosed", "echo \"a'b", "echo \"a\" \"", "'",
    "\"", "''", "\"\"", "export A=$HOME", "export A", "export \"A\"", "export 'A=$B' C=$A", "export A=\"$B\" X", "export $A", "export",
    "export  A=1  B", "echo export A", "x export A=$A", "export A='x y'", "export \"$A\"=1", "echo \t a \x0b b \x0c c \r d", " echo   x   ",
    "echo\ta", "echo日本語 $A", "echo \"日本 $USER\"", "echo $日本", "echo ñ$HOME", "echo >>>> f", "echo <<< f", "echo <><> f", "echo a>b>c",
    "echo a<b<c", "echo \"a|b\" 'c>d'", "echo a\"b c\"d", "echo \"a\"\"b\"", "echo 'a''b'", "echo \"$A$B\"x", "echo '$A'\"$B\"'$A'", "$A", "$A $B",
    "$NOSUCH", "\"$NOSUCH\"", "echo $A.$B-$USER_", "echo $USER_x", "echo \"$ A\"", "echo \"$$\"", "echo \"$?x\"", "echo $?$?", "echo $?x",
    "cat <$A", "cat > \"$HOME/f\"", "cat > '$HOME/f'", "cat << '$A'", "cat << \"$A\"", "ls | grep $A | wc -l > $B",
]

VARS = ["$HOME", "$USER", "$A", "$B", "$EMPTY", "$NOVAL", "$NOSUCH", "$?", "$$", "$", "$1", "$_UND", "$x1", "$EQ", "$PATH", "${A}",
        "$A$B", "$é", "$A.", "$A-x", "$USER_", "$9a"]
PLAIN = ["echo", "ls", "cat", "grep", "export", "a", "foo=bar", "A=$A", "-n", "x1", "path/to/f", "é", "日本", "a=b=c", "$", "~", "*", "!", "#", "%", "&", "(", ")"]
OPS = ["|", ">", ">>", "<", "<<"]
BAD = [";", "\\", "||", ">>>", "<<<", "|>", "><"]
WS = [" ", "  ", "\t", " \t ", "\x0b", "\x0c", "\r"]


def quoted(rng):
    body = "".join(rng.choice([rng.choice(VARS), rng.choice(PLAIN), " ", "  ", "|", ">", "<", ";", "\\", "'", '"']) for _ in range(rng.randint(0, 4)))
    q = rng.choice(["'", '"'])
    if q in body:
        body = body.replace(q, "x")
    return q + body + q


def word(rng):
    parts = []
    for _ in range(rng.randint(1, 3)):
        r = rng.random()
        parts.append(rng.choice(VARS) if r < .3 else quoted(rng) if r < .6 else rng.choice(PLAIN))
    return "".join(parts)


def gen(rng):
    toks = []
    for _ in range(rng.randint(1, 8)):
        r = rng.random()
        toks.append(rng.choice(OPS) if r < .2 else rng.choice(BAD) if r < .25 else word(rng))
    sep = lambda: rng.choice(WS) if rng.random() < .4 else " "
    s = "".join(t + (sep() if rng.random() < .85 else "") for t in toks)
    if rng.random() < .1:
        s += rng.choice(["'", '"'])
    if rng.random() < .1:
        s = sep() + s
    return s.replace("\n", " ")


def main(n=400, seed=20261115):
    rng = random.Random(seed)
    seen, out = set(), []
    for s in HAND + [gen(rng) for _ in range(n * 3)]:
        if s not in seen:
            seen.add(s)
            out.append(s)
        if len(out) >= n + len(HAND):
            break
    OUT.write_text("\n".join(out) + "\n")
    print(len(out), "inputs ->", OUT)


if __name__ == "__main__":
    main()
