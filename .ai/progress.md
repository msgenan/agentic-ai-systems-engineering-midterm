# Midterm project — progress

Course: YBS315 Ajentik Yapay Zeka Sistem Mühendisliği. Deliverable: one PDF, max 10 pages excl. appendices.
Due: 2026-11-15 23:59. Grading: literature review 50%, test report 40%, AI-use statement 10%.

## Decisions
- Domain: software engineering, sub-topic code migration (language translation + version/framework/API upgrade).
- Focus question (draft): LLM-based agents in codebase migration — where do they add value over fixed workflows,
  and how is behavioural equivalence verified?
- Expected argument (to be confirmed after reading): most industry "agent" migration successes are workflows
  (LLM translates, compiler/tests verify, human reviews); open gap = verifying behavioural equivalence.
- Demo: Type B. Small C module -> Rust. Baseline = single-shot translation; agent = translate/build/test/fix loop
  via `claude -p`. Variations on 4-5 case subset: model (Sonnet vs Haiku), instruction version, tool removed.
- Roles: Anakin finds/verifies sources, builds demo, drafts cases, report skeleton, language edits.
  Owner reads every cited source, approves cases and verdicts, writes report text and AI-use statement.
  Argument is discussed together; owner decides.

## Steps (status)
1. Setup: folder, git, Rust (cargo 1.100) — DONE 2026-11-14
2. Source list — DONE: owner approved 9 (research/sources.md 'Önerilen seçim'); Scholar/Semantic Scholar queries still to add to search log
3. Reading (owner) — IN PROGRESS: start with Anthropic 2024 + Pan et al. 2024
4. Demo build — DONE: demo/run.py harness, 14 cases (demo/cases/), prompts; references pass, expected values checked against gcc-compiled C; smoke test ok (deleted)
5. Test cases — WAITING owner approval: demo/cases.md
6. Runs + measurement — todo
7. Discussion -> 4-5 bullet argument — todo
8. Writing (owner) + skeleton/tables/diagram/APA — todo
9. PDF + final checks — todo

## AI output corrected/rejected by owner (for AI-use statement)
- (none yet)
