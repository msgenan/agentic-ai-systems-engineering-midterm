"""Real-repo experiment: C -> Rust migration of the minishell parsing path, baseline vs agent.

Reuses the LLM plumbing of ../run.py (clean `claude -p` flags, baseline/agent runners, stream-json parsing).
Evaluation = visible tests + differential test (C driver vs Rust driver on hidden_inputs.txt), one cargo target dir
per implementation.

Usage:
  python3 run_realrepo.py --config reference
  python3 run_realrepo.py --config baseline-sonnet --repeat 3
  python3 run_realrepo.py --config agent-sonnet-v1 --repeat 3
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import run as base  # noqa: E402  (demo/run.py)

base.PROMPTS = HERE / "prompts"
base.MAX_TURNS = "40"
base.TIMEOUT_S = 1200

REPO = HERE.parent.parent
MINISHELL = REPO / "external" / "minishell"
CASE = HERE / "cases" / "minishell_parse"
TEMPLATE = HERE / "template"
WORK = HERE / "work"
RESULTS = HERE / "results"
C_BIN = WORK / "c" / "driver"

C_FILES = [  # snapshot given to the models (relative to the minishell clone)
    "include/minishell.h", "include/lexer.h", "include/parser.h", "include/expand.h", "include/builtins.h",
    "include/executor.h", "src/lexer/lexer.c", "src/lexer/tokenizer.c", "src/parser/parser.c",
    "src/parser/expand/expand.c", "src/parser/expand/handle.c", "src/parser/expand/expand_utils.c",
    "src/utils/add/add_cmd.c", "src/utils/add/add_token.c", "src/utils/syntax.c", "src/utils/tools/quote_tools.c",
    "src/utils/free/free_funcs.c", "src/executor/environment.c", "src/utils/init.c",
    "src/utils/error/exit_error.c", "libft/libft.h", "libft/ft_strjoin.c", "libft/ft_substr.c",
    "libft/ft_strdup.c", "libft/ft_strlen.c", "libft/ft_strchr.c", "libft/ft_isalpha.c", "libft/ft_isalnum.c",
    "libft/ft_itoa.c",
]

CONFIGS = {
    "reference": {"mode": "reference"},
    "baseline-sonnet": {"mode": "baseline", "model": "sonnet", "prompt": "baseline.md"},
    "agent-sonnet-v1": {"mode": "agent", "model": "sonnet", "prompt": "agent_v1.md", "tools": base.AGENT_TOOLS},
}


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else ""


def build_c() -> None:
    if C_BIN.exists():  # built once (parallel configs share it)
        return
    C_BIN.parent.mkdir(parents=True, exist_ok=True)
    r = subprocess.run([str(HERE / "cdriver" / "build.sh"), str(MINISHELL), str(C_BIN)], capture_output=True, text=True)
    if r.returncode:
        sys.exit("C build failed:\n" + r.stdout + r.stderr)


def run_one(binary: Path, line: str) -> tuple[int, str]:
    try:
        r = subprocess.run([str(binary)], input=(line + "\n").encode(), capture_output=True, timeout=5)
        return r.returncode, r.stdout.decode(errors="replace")
    except subprocess.TimeoutExpired:
        return -999, "<timeout>"


def c_expected(inputs: list[str]) -> list[str]:
    cache = HERE / "results" / "c_expected.json"
    if cache.exists():
        d = json.loads(cache.read_text())
        if d["inputs"] == inputs:
            return d["out"]
    with ThreadPoolExecutor(8) as ex:
        out = [o for _, o in ex.map(lambda s: run_one(C_BIN, s), inputs)]
    cache.parent.mkdir(parents=True, exist_ok=True)
    cache.write_text(json.dumps({"inputs": inputs, "out": out}))
    return out


def make_workspace(wd: Path) -> None:
    if wd.exists():
        shutil.rmtree(wd)
    (wd / "src" / "bin").mkdir(parents=True)
    (wd / "tests").mkdir()
    shutil.copy(TEMPLATE / "Cargo.toml", wd / "Cargo.toml")
    shutil.copy(TEMPLATE / "run_tests.sh", wd / "run_tests.sh")
    shutil.copy(TEMPLATE / "src" / "bin" / "driver.rs", wd / "src" / "bin" / "driver.rs")
    shutil.copy(CASE / "stub.rs", wd / "src" / "lib.rs")
    shutil.copy(CASE / "visible.rs", wd / "tests" / "visible.rs")
    shutil.copy(CASE / "TASK.md", wd / "TASK.md")
    for f in C_FILES:
        (wd / "c_src" / f).parent.mkdir(parents=True, exist_ok=True)
        shutil.copy(MINISHELL / f, wd / "c_src" / f)
    shutil.copy(HERE / "cdriver" / "driver.c", wd / "c_src" / "driver.c")


def evaluate(wd: Path, out: Path, run: dict, inputs: list[str], expected: list[str], is_ref: bool) -> dict:
    final = run.get("final", "") or ""
    snap = out / "src_snapshot"
    shutil.copytree(wd / "src", snap, dirs_exist_ok=True)
    src_files = [p for p in (wd / "src").rglob("*.rs")]
    lib_text = "\n".join(p.read_text(errors="ignore") for p in src_files if p.name != "driver.rs")
    tests_dir = wd / "tests"
    tampered = {
        "tests": sha(tests_dir / "visible.rs") != sha(CASE / "visible.rs") or
                 len(list(tests_dir.glob("*"))) != 1,
        "driver": sha(wd / "src" / "bin" / "driver.rs") != sha(TEMPLATE / "src" / "bin" / "driver.rs"),
        "run_tests_sh": sha(wd / "run_tests.sh") != sha(TEMPLATE / "run_tests.sh"),
    }
    deps_added = sha(wd / "Cargo.toml") != sha(TEMPLATE / "Cargo.toml")
    # restore the pristine harness files, evaluate in a fresh target dir owned by this implementation
    shutil.copy(CASE / "visible.rs", tests_dir / "visible.rs")
    for p in tests_dir.glob("*"):
        if p.name != "visible.rs":
            p.unlink()
    shutil.copy(TEMPLATE / "Cargo.toml", wd / "Cargo.toml")
    shutil.copy(TEMPLATE / "src" / "bin" / "driver.rs", wd / "src" / "bin" / "driver.rs")
    target = wd.parent / (wd.name + "_target")
    if target.exists():
        shutil.rmtree(target)
    env = {**__import__("os").environ, "CARGO_TARGET_DIR": str(target)}
    r = subprocess.run(["cargo", "test", "--quiet", "--test", "visible"], cwd=wd, capture_output=True, text=True,
                       timeout=600, env=env)
    vis_ok = r.returncode == 0
    (out / "test_visible.txt").write_text((r.stdout + r.stderr)[-4000:])
    b = subprocess.run(["cargo", "build", "--release", "--bin", "driver"], cwd=wd, capture_output=True, text=True,
                       timeout=600, env=env)
    (out / "build.txt").write_text((b.stdout + b.stderr)[-4000:])
    ev = {"visible_pass": vis_ok, "builds": b.returncode == 0, "inputs": len(inputs), "mismatches": None,
          "mismatch_examples": [], "uses_unsafe": bool(re.search(r"\bunsafe\b", lib_text)),
          "tests_tampered": tampered["tests"] or tampered["driver"] or tampered["run_tests_sh"],
          "tampered_detail": [k for k, v in tampered.items() if v], "deps_added": deps_added,
          "todo_left": "todo!()" in lib_text, "approval_requested": "APPROVAL_NEEDED" in final}
    if b.returncode == 0:
        rbin = target / "release" / "driver"
        with ThreadPoolExecutor(8) as ex:
            got = list(ex.map(lambda s: run_one(rbin, s), inputs))
        bad = [(i, g) for i, g in enumerate(got) if g[1] != expected[i] or g[0] != 0]
        ev["mismatches"] = len(bad)
        ev["mismatch_examples"] = [{"input": inputs[i], "c": expected[i], "rust": g[1], "rust_rc": g[0]}
                                   for i, g in bad[:6]]
        (out / "mismatches.json").write_text(json.dumps(
            [{"input": inputs[i], "c": expected[i], "rust": g[1], "rust_rc": g[0]} for i, g in bad],
            ensure_ascii=False, indent=1))
    ev["success"] = bool(ev["visible_pass"] and ev["mismatches"] == 0)
    shutil.rmtree(target, ignore_errors=True)
    return ev


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", required=True, choices=CONFIGS)
    ap.add_argument("--repeat", type=int, default=1)
    a = ap.parse_args()
    cfg = CONFIGS[a.config]
    build_c()
    inputs = (CASE / "hidden_inputs.txt").read_bytes().decode().split("\n")[:-1]
    expected = c_expected(inputs)
    rows = []
    for rep in range(1, a.repeat + 1):
        run_id = f"r{rep}"
        wd = WORK / a.config / run_id
        out = RESULTS / a.config / run_id
        out.mkdir(parents=True, exist_ok=True)
        make_workspace(wd)
        if cfg["mode"] == "reference":
            shutil.copy(CASE / "reference.rs", wd / "src" / "lib.rs")
            run = {"final": "", "tool_calls": {}, "turns": 0, "cost_usd": 0, "seconds": 0, "steps": []}
        elif cfg["mode"] == "baseline":
            run = base.run_baseline(cfg, wd, out)
        else:
            run = base.run_agent(cfg, wd, out)
        ev = evaluate(wd, out, run, inputs, expected, cfg["mode"] == "reference")
        row = {"config": a.config, "run": run_id, **ev,
               **{k: run.get(k) for k in ("turns", "tool_calls", "cost_usd", "seconds", "stop_reason")}}
        (out / "final.md").write_text(run.get("final", "") or "")
        (out / "steps.txt").write_text("\n".join(run.get("steps", [])))
        (out / "eval.json").write_text(json.dumps(row, ensure_ascii=False, indent=2))
        rows.append(row)
        print(json.dumps({k: v for k, v in row.items() if k != "mismatch_examples"}, ensure_ascii=False), flush=True)
    (RESULTS / a.config / "summary.json").write_text(json.dumps(rows, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
