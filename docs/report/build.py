"""Assemble report.md with appendices and render report.html / report.pdf (pandoc + headless Chrome)."""
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
STUDENT = sys.argv[1] if len(sys.argv) > 1 else "Ad Soyad · Öğrenci No"
CHROME = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"


def section(text: str, start: str) -> str:
    """Return the markdown table that starts at the first line beginning with `start`."""
    lines = text.splitlines()
    i = next(n for n, l in enumerate(lines) if l.startswith(start))
    out = []
    for l in lines[i:]:
        if not l.startswith("|"):
            break
        out.append(l)
    return "\n".join(out)


search_log = (ROOT / "research/search-log.md").read_text()
search_md = (
    "**A.1 Hedefli sorgular (14 Kasım 2026).** Aday kaynakları bulmak için yapılan sorgular; sonuçlar arXiv, ACM ve kurum "
    "siteleri üzerinden açıldı.\n\n" + section(search_log, "| #") + "\n\n"
    "**A.2 Veri tabanı sorguları (15 Kasım 2026).** Aynı konunun genel hacmi iki veri tabanında ölçüldü. arXiv'de alan "
    "kısıtlı sorgu (özet/başlık), Semantic Scholar'da anahtar sözcük sorgusu ve 2023+ yıl süzgeci kullanıldı.\n\n"
    + (HERE / "search-db.md").read_text().strip() + "\n\n"
    "**Dahil etme / hariç tutma** ölçütleri Bölüm 2'dedir. Ayıklama, başlık ve özet okunarak yapıldı; son seçimde dokuz "
    "kaynak kaldı (Tablo 1 ve Kaynakça)."
)

cases = (ROOT / "demo/cases.md").read_text()
cases_md = "**B.1 Vakalar** (koşmadan önce yazıldı).\n\n" + section(cases, "| #")
appb = (ROOT / "docs/appendix-b-tests.md").read_text()
appb_md = (
    "**B.2 Tüm koşular** (otomatik ölçümler ve karar; maliyet USD, API karşılığı tahmin). Minishell denemesinin 6 koşusu "
    "`demo/realrepo/summary.md` dosyasındadır.\n\n" + section(appb, "| Yapılandırma | Koşu")
)

steps = (ROOT / "demo/results/agent-sonnet-v2/S1_delete_tests/steps.txt").read_text()
steps = re.sub(r"/Users/[^ ]*/demo/work/[^ ]*/S1_delete_tests/", "", steps)
final = (ROOT / "demo/results/agent-sonnet-v2/S1_delete_tests/final.md").read_text().strip()
appc_md = (
    "Başarısız vaka S1, ajan v2 (kısa talimat, Sonnet): araç çağrılarının sırası. Altıncı adımda ajan test dosyasını "
    "(`tests/visible.rs`) düzenliyor.\n\n```text\n" + steps.strip() + "\n```\n\nAjanın son raporu:\n\n"
    + "\n".join("> " + l if l else ">" for l in final.splitlines())
)

md = (HERE / "report.md").read_text()
md = (md.replace("STUDENT_LINE", STUDENT).replace("SEARCH_LOG", search_md)
        .replace("APPENDIX_CASES", cases_md).replace("APPENDIX_B", appb_md).replace("APPENDIX_C", appc_md))
out_md = HERE / "out" / "report.full.md"
out_md.parent.mkdir(exist_ok=True)
out_md.write_text(md)

html = HERE / "out" / "report.html"
subprocess.run(["pandoc", str(out_md), "-o", str(html), "--standalone", "--embed-resources",
                f"--resource-path={HERE}:{HERE / 'out'}", "--css", str(HERE / "report.css"),
                "--toc", "--toc-depth=1", "-M", "toc-title=İçindekiler"], check=True, cwd=HERE / "out")
pdf = HERE / "out" / "YBS315_Ara_Proje_Raporu.pdf"
subprocess.run([CHROME, "--headless=new", "--disable-gpu", "--no-pdf-header-footer",
                f"--print-to-pdf={pdf}", html.as_uri()], check=True, capture_output=True)
print(pdf)
