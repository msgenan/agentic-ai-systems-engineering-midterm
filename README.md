# Agentic AI Systems Engineering — Midterm Project

YBS315 Ajentik Yapay Zekâ Sistem Mühendisliği, ara sınav projesi (İstinye Üniversitesi, Güz 2026).

**Konu:** Kod tabanı göçünde LLM ajanları. C'den Rust'a çeviride tek çağrılı temel sistem ile Claude Code ajanının
karşılaştırılması (Tür B) ve diferansiyel testle davranış eşdeğerliğinin doğrulanması.

- Rapor: [`docs/report/out/YBS315_Ara_Proje_Raporu.pdf`](docs/report/out/YBS315_Ara_Proje_Raporu.pdf) (kaynak `docs/report/report.md`, derleme `docs/report/build.py`)
- Test vakaları: `demo/cases.md`, `demo/cases/`
- Koşu düzeneği: `demo/run.py`; tüm koşu kayıtları (adımlar, son rapor, transcript): `demo/results/`
- Sonuç özeti: `demo/results-summary.md`, tam tablo `docs/appendix-b-tests.md`
- Diferansiyel test: `demo/difftest.py`, `demo/difftest/report.json`
- Gerçek proje denemesi (minishell ayrıştırma yolu): `demo/realrepo/`
- Kaynak notları ve arama kaydı: `research/`

Gereksinimler: Rust (cargo), gcc, Python 3.12, Claude Code CLI (`claude -p`).
