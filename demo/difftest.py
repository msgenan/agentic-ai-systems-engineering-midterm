"""Differential testing: original C (gcc -O0 and -O2) vs every generated Rust translation, on seeded random inputs.

Inputs are tagged "in" (inside the task contract) or "out" (outside it, e.g. C undefined behaviour such as signed
overflow). Usage: python3 difftest.py [--n 400]   -> writes difftest/report.json and difftest/report.md
"""
from __future__ import annotations

import argparse
import json
import random
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent
CASES = ROOT / "cases"
RESULTS = ROOT / "results"
OUT = ROOT / "difftest"
BUILD = ROOT / "work" / "difftest"


def hx(s: str) -> str:
    return s.encode().hex()


# ---------- input generators: return list of (tag, line) ----------
def g_sum(rng, n):
    ext = [0, 1, -1, 2**31 - 1, -(2**31)]
    res = []
    for _ in range(n):
        k = rng.randint(0, 20)
        res.append(("in", " ".join(str(rng.choice(ext) if rng.random() < .3 else rng.randint(-2**31, 2**31 - 1))
                                   for _ in range(k))))
    return res


WS = [" ", "\t", "\n", "\x0b", "\x0c", "\r"]
UNI = ["é", " ", "　", "ş", "日"]


def g_words(rng, n):
    res = []
    for _ in range(n):
        s = "".join(rng.choice(WS) if rng.random() < .3 else rng.choice("abcXYZ019.,") for _ in range(rng.randint(0, 25)))
        tag = "in"
        if rng.random() < .25:
            pos = rng.randint(0, len(s))
            s = s[:pos] + rng.choice(UNI) + s[pos:]
        res.append((tag, hx(s)))
    return res


def g_checksum(rng, n):
    return [("in", bytes(rng.randint(0, 255) for _ in range(rng.randint(0, 200))).hex()) for _ in range(n)]


def g_parse(rng, n):
    res = []
    for _ in range(n):
        out = rng.random() < .2
        pre = "".join(rng.choice(WS + ["x", "+", "-"]) for _ in range(rng.randint(0, 3)))
        digits = "".join(rng.choice("0123456789") for _ in range(rng.randint(10, 20) if out else rng.randint(0, 9)))
        post = "".join(rng.choice(" abc-+12") for _ in range(rng.randint(0, 4)))
        text = pre + digits + post
        res.append(("out" if c_parse_overflows(text) else "in", hx(text)))
    return res


def c_parse_overflows(s: str) -> bool:
    """True if the C parse_int would overflow int (undefined behaviour) on this input."""
    i = 0
    while i < len(s) and s[i] in " \t\n\x0b\x0c\r":
        i += 1
    if i < len(s) and s[i] in "+-":
        i += 1
    v = 0
    while i < len(s) and s[i].isdigit():
        v = v * 10 + int(s[i])
        if v > 2**31 - 1:
            return True
        i += 1
    return False


def g_prime(rng, n):
    res = []
    for _ in range(n):
        if rng.random() < .1:
            res.append(("out", str(rng.randint(46341 * 46340, 2**31 - 1))))  # i*i may overflow int in C (UB)
        else:
            res.append(("in", str(rng.randint(-1000, 46340 * 46340))))
    res.append(("out", str(2**31 - 1)))
    return res


def g_code(rng, n):
    res = []
    for _ in range(n):
        if rng.random() < .5:
            s = "".join(rng.choice("ABCXYZ") for _ in range(3)) + "-" + "".join(rng.choice("0123456789") for _ in range(4))
            if rng.random() < .5:
                i = rng.randint(0, len(s) - 1)
                s = s[:i] + rng.choice("aZ5-\n é") + s[i + 1:]
        else:
            s = "".join(rng.choice("ABCabc0123-\n ") for _ in range(rng.randint(0, 10)))
        res.append(("in", hx(s)))
    return res


def expr(rng, d):
    r = rng.random()
    if d <= 0 or r < .3:
        return str(rng.randint(0, 99))
    if r < .45:
        return "(" + expr(rng, d - 1) + ")"
    if r < .55:
        return "-" + expr(rng, d - 1)
    return expr(rng, d - 1) + rng.choice(["+", "-", "*", "/", "%", " + ", " * ", " / "]) + expr(rng, d - 1)


def g_eval(rng, n):
    res = []
    for _ in range(n):
        s = expr(rng, rng.randint(0, 4))
        if rng.random() < .3 and s:
            i = rng.randint(0, len(s))
            s = s[:i] + rng.choice(["a", ")", "(", "*", "", " ", "--"]) + s[i + 1:]
        res.append(("in", hx(s)))
    return res


def g_table(rng, n):
    keys = [f"k{i}" for i in range(12)] + ["apple", "zeta", "q", "longer_key"]
    res = []
    for _ in range(n):
        ops = []
        for _ in range(rng.randint(1, 60)):
            k = rng.choice(keys)
            r = rng.random()
            ops.append(f"p {k} {rng.randint(-50, 50)}" if r < .5 else (f"g {k}" if r < .75 else f"d {k}"))
        res.append(("in", ";".join(ops)))
    return res


# ---------- C drivers ----------
C_HEAD = r"""#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
static size_t unhex(const char *h, unsigned char *b) { size_t n = strlen(h) / 2;
  for (size_t i = 0; i < n; i++) { unsigned v; sscanf(h + 2 * i, "%2x", &v); b[i] = (unsigned char)v; } return n; }
static char line[1 << 16]; static unsigned char buf[1 << 15];
static int next_line(void) { if (!fgets(line, sizeof line, stdin)) return 0; line[strcspn(line, "\n")] = 0; return 1; }
"""

C_MAIN = {
    "T1_sum_array": r"""long sum_array(const int *a, size_t n);
int main(void) { int a[64]; while (next_line()) { size_t n = 0; char *p = strtok(line, " ");
  while (p) { a[n++] = (int)strtol(p, 0, 10); p = strtok(0, " "); } printf("%ld\n", sum_array(a, n)); } }""",
    "T3_count_words": r"""int count_words(const char *s);
int main(void) { while (next_line()) { size_t n = unhex(line, buf); buf[n] = 0; printf("%d\n", count_words((char *)buf)); } }""",
    "H1_checksum": r"""uint32_t checksum(const unsigned char *d, size_t n);
int main(void) { while (next_line()) { size_t n = unhex(line, buf); printf("%u\n", checksum(buf, n)); } }""",
    "H2_parse_int": r"""int parse_int(const char *s);
int main(void) { while (next_line()) { size_t n = unhex(line, buf); buf[n] = 0; printf("%d\n", parse_int((char *)buf)); } }""",
    "A1_tool_failure": r"""int is_prime(int n);
int main(void) { while (next_line()) { printf("%d\n", is_prime((int)strtol(line, 0, 10))); } }""",
    "A2_needs_dependency": r"""int is_valid_code(const char *s);
int main(void) { while (next_line()) { size_t n = unhex(line, buf); buf[n] = 0; printf("%d\n", is_valid_code((char *)buf)); } }""",
    "R1_expr_eval": r"""int eval(const char *s, long *out, size_t *err_pos);
int main(void) { while (next_line()) { size_t n = unhex(line, buf); buf[n] = 0; long v = 0; size_t p = 0;
  int r = eval((char *)buf, &v, &p); if (r == 0) printf("OK %ld\n", v); else printf("ERR %d %zu\n", r, p); } }""",
    "R2_hash_table": r"""typedef struct Table Table; Table *ht_new(void); void ht_put(Table*, const char*, int);
int ht_get(const Table*, const char*, int*); int ht_delete(Table*, const char*); size_t ht_len(const Table*);
size_t ht_capacity(const Table*); size_t ht_keys(const Table*, const char**, size_t); void ht_free(Table*);
int main(void) { while (next_line()) { Table *t = ht_new(); char *save; char *op = strtok_r(line, ";", &save);
  while (op) { char k[64]; int v; if (op[0] == 'p') { sscanf(op + 2, "%63s %d", k, &v); ht_put(t, k, v); }
    else if (op[0] == 'g') { sscanf(op + 2, "%63s", k); if (ht_get(t, k, &v)) printf("%d,", v); else printf("N,"); }
    else { sscanf(op + 2, "%63s", k); printf("%d,", ht_delete(t, k)); } op = strtok_r(0, ";", &save); }
  const char *ks[256]; size_t n = ht_keys(t, ks, 256); printf("|%zu %zu ", ht_len(t), ht_capacity(t));
  for (size_t i = 0; i < n; i++) printf("%s,", ks[i]); printf("\n"); ht_free(t); } }""",
}

# ---------- Rust drivers (body of fn run(line: &str) -> String) ----------
R_RUN = {
    "T1_sum_array": 'let a: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect(); format!("{}", sum_array(&a))',
    "T3_count_words": 'format!("{}", count_words(&String::from_utf8(unhex(line)).unwrap()))',
    "H1_checksum": 'format!("{}", checksum(&unhex(line)))',
    "H2_parse_int": 'format!("{}", parse_int(&String::from_utf8(unhex(line)).unwrap()))',
    "A1_tool_failure": 'format!("{}", is_prime(line.trim().parse().unwrap()) as i32)',
    "A2_needs_dependency": 'format!("{}", is_valid_code(&String::from_utf8(unhex(line)).unwrap()) as i32)',
    "R1_expr_eval": '''match eval(&String::from_utf8(unhex(line)).unwrap()) {
        Ok(v) => format!("OK {}", v),
        Err((e, p)) => format!("ERR {} {}", if e == EvalError::Syntax { 1 } else { 2 }, p) }''',
    "R2_hash_table": '''let mut t = Table::new(); let mut o = String::new();
    for op in line.split(';') { let parts: Vec<&str> = op.split(' ').collect();
        match parts[0] { "p" => t.put(parts[1], parts[2].parse().unwrap()),
            "g" => o.push_str(&match t.get(parts[1]) { Some(v) => format!("{},", v), None => "N,".to_string() }),
            _ => o.push_str(&format!("{},", t.delete(parts[1]) as i32)) } }
    o.push_str(&format!("|{} {} ", t.len(), t.capacity()));
    for k in t.keys() { o.push_str(&k); o.push(','); } o''',
}

R_MAIN = r"""use migrated::*;
use std::io::{self, BufRead, Write};
#[allow(dead_code)]
fn unhex(s: &str) -> Vec<u8> { (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()).collect() }
fn run(line: &str) -> String { BODY }
fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let mut out = io::BufWriter::new(io::stdout());
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        match std::panic::catch_unwind(|| run(&line)) { Ok(s) => writeln!(out, "{}", s).unwrap(),
            Err(_) => writeln!(out, "PANIC").unwrap() }
    }
}
"""

GEN = {"T1_sum_array": g_sum, "T3_count_words": g_words, "H1_checksum": g_checksum, "H2_parse_int": g_parse,
       "A1_tool_failure": g_prime, "A2_needs_dependency": g_code, "R1_expr_eval": g_eval, "R2_hash_table": g_table}


def sh(cmd, cwd=None, inp=None, timeout=600):
    r = subprocess.run(cmd, cwd=cwd, input=inp, capture_output=True, text=True, timeout=timeout)
    if r.returncode != 0:
        raise RuntimeError(f"{cmd}: {r.stderr[-800:]}")
    return r.stdout


def c_outputs(case: str, inputs: str, opt: str) -> list[str]:
    d = BUILD / "c" / case
    d.mkdir(parents=True, exist_ok=True)
    (d / "driver.c").write_text(C_HEAD + C_MAIN[case])
    exe = d / f"drv{opt}"
    sh(["gcc", opt, "-w", "-o", str(exe), str(d / "driver.c"), str(CASES / case / "original.c")])
    try:
        return sh([str(exe)], inp=inputs, timeout=300).splitlines()
    except subprocess.TimeoutExpired:
        return ["TIMEOUT"] * len(inputs.splitlines())


def rust_outputs(case: str, lib: Path, inputs: str, label: str = "adhoc") -> list[str]:
    # One project dir per implementation: reusing a dir can make cargo keep a stale binary
    # when lib.rs is replaced within the same second.
    d = BUILD / "rust" / case / label.replace("/", "__")
    (d / "src" / "bin").mkdir(parents=True, exist_ok=True)
    shutil.copy(ROOT / "template" / "Cargo.toml", d / "Cargo.toml")
    shutil.copy(lib, d / "src" / "lib.rs")
    (d / "src" / "bin" / "drive.rs").write_text(R_MAIN.replace("BODY", R_RUN[case]))
    sh(["cargo", "build", "--quiet", "--bin", "drive"], cwd=d)
    return sh([str(d / "target" / "debug" / "drive")], inp=inputs, timeout=300).splitlines()


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--n", type=int, default=400)
    ap.add_argument("--cases", default="")
    a = ap.parse_args()
    OUT.mkdir(exist_ok=True)
    only = set(a.cases.split(",")) if a.cases else set(GEN)
    old = json.loads((OUT / "report.json").read_text()) if (OUT / "report.json").exists() else []
    report = [r for r in old if r["case"] not in only]
    for case, gen in GEN.items():
        if case not in only:
            continue
        rng = random.Random(f"seed-{case}")
        items = gen(rng, a.n)
        inputs = "\n".join(line for _, line in items) + "\n"
        tags = [t for t, _ in items]
        c0, c2 = c_outputs(case, inputs, "-O0"), c_outputs(case, inputs, "-O2")
        c_disagree = [i for i in range(len(items)) if c0[i] != c2[i]]
        libs = sorted(RESULTS.glob(f"*/{case}*/lib.rs")) + [CASES / case / "reference.rs"]
        for lib in libs:
            label = "reference" if lib.parent.name == case and lib.parent.parent == CASES else f"{lib.parent.parent.name}/{lib.parent.name}"
            try:
                ro = rust_outputs(case, lib, inputs, label)
            except RuntimeError as e:
                report.append({"case": case, "impl": label, "error": str(e)[:300]})
                continue
            mism = {"in": 0, "out": 0}
            panics = {"in": 0, "out": 0}
            ex = []
            for i, t in enumerate(tags):
                if ro[i] != c0[i]:
                    mism[t] += 1
                    panics[t] += ro[i] == "PANIC"
                    if len(ex) < 3:
                        ex.append({"tag": t, "input": items[i][1][:80], "c_O0": c0[i][:80], "c_O2": c2[i][:80], "rust": ro[i][:80]})
            report.append({"case": case, "impl": label, "n_in": tags.count("in"), "n_out": tags.count("out"),
                           "mismatch_in": mism["in"], "mismatch_out": mism["out"], "panic_in": panics["in"],
                           "panic_out": panics["out"], "c_O0_vs_O2_disagree": len(c_disagree), "examples": ex})
            print(json.dumps(report[-1], ensure_ascii=False)[:300], flush=True)
    (OUT / "report.json").write_text(json.dumps(report, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
