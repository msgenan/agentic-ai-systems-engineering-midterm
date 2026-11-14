"""Prepare CRUST-Bench task workspaces (no deletion; copies only).

Usage: python3 crust_prep.py <dest_dir> <project> [<project> ...]
Layout produced per project:
  <dest>/<project>/Cargo.toml, src/lib.rs, src/<module>.rs (interface stubs),
  src/bin/*.rs (CRUST tests), c_src/ (original C files), TASK.md, run_tests.sh
"""
from __future__ import annotations

import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent / "external" / "crust-bench"
RUN_TESTS = "#!/bin/sh\n# Runs the CRUST-Bench test suite of this project.\ncargo test --quiet 2>&1 | tail -60\n"
TASK = (
    "Bu proje bir C kod tabanının Rust'a göçüdür. `c_src/` altındaki C kodunu, `src/` altındaki Rust arayüz "
    "dosyalarındaki `unimplemented!()` gövdeleri doldurarak güvenli Rust'a çevir. Fonksiyon imzalarını değiştirme. "
    "Testler `src/bin/` altında; davranış C koduyla aynı olmalı.\n"
)


def c_dir(project: str) -> Path:
    for d in (ROOT / "CBench").iterdir():
        if d.name.lower() == project.lower():
            return d
    raise SystemExit(f"C project not found: {project}")


def prepare(dest: Path, project: str) -> Path:
    src = ROOT / "RBench" / project
    wd = dest / project
    if wd.exists():
        raise SystemExit(f"refusing to overwrite existing workspace: {wd}")
    shutil.copytree(src, wd, ignore=shutil.ignore_patterns(".git", "target", "interfaces", "Cargo.lock"))
    for f in (src / "src" / "interfaces").glob("*.rs"):
        shutil.copy(f, wd / "src" / f.name)
    shutil.copytree(c_dir(project), wd / "c_src", ignore=shutil.ignore_patterns(".git"))
    (wd / "run_tests.sh").write_text(RUN_TESTS)
    (wd / "run_tests.sh").chmod(0o755)
    (wd / "TASK.md").write_text(TASK)
    return wd


if __name__ == "__main__":
    dest = Path(sys.argv[1])
    dest.mkdir(parents=True, exist_ok=True)
    for p in sys.argv[2:]:
        print(prepare(dest, p))
