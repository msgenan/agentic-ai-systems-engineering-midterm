# Rapor iskeleti (metni sen yazarsın)

Sınır: ekler hariç 10 sayfa. Puan: literatür %50, test raporu %40, AI beyanı %10.
Aşağıdaki maddeler yazarken dayanacağın olgular ve sorulardır; cümleler senin olmalı.
Sayfa bütçesi parantez içinde.

## Kapak (sayılmaz sayılırsa 0,25)
Başlık önerisi: "Kod Tabanı Göçünde LLM Ajanları: Ne Zaman Gerekli?" — ders, ad, tarih.

## 1. Giriş (0,75)
- Alan: yazılım mühendisliği, kod göçü (dil çevirisi + sürüm/çerçeve yükseltme).
- Odak sorusu (son hali sende): LLM tabanlı ajanlar kod tabanı göçünde sabit iş akışlarına göre hangi noktalarda değer katıyor ve davranışın korunduğu nasıl doğrulanıyor?
- Neden önemli: eski kod tabanları, bellek güvenliği (DARPA TRACTOR yedek kaynak), şirketlerin göç maliyeti (Google, Airbnb örnekleri).
- Sınıflandırma çerçevesi: Anthropic (2024) — LLM uygulaması / iş akışı / ajan; yönergenin dört sorusu.

## 2. Yöntem (1,25)
- Veri tabanları: arXiv + Google Scholar / Semantic Scholar (ikincisini çalıştırıp kaydet). Arama terimleri: research/search-log.md.
- Dahil etme: 2023+, LLM tabanlı kod çevirisi/göçü, sonuç raporlayan; hariç: yalnız BLEU raporlayan, LLM öncesi.
- Her sistem için: temel model, RAG/ince ayar, çerçeve, araçlar → özet tabloya (bölüm 4).
- Arama kaydı → Ek A.

## 3. Alandaki ajan tabanlı uygulamalar (2,0)
Her biri için: işlev, araçlar/eylemler, özerklik düzeyi (dört soru), kanıt, sınırlılık.
- Tek istem (LLM uygulaması): Pan vd. 2024 — hata kategorileri; temel sistemin karşılığı.
- Endüstri iş akışları: Nikolov vd. 2025 (Google), Covey-Brandt 2025 (Airbnb) — "ajan" deniyor, adımları kod mu sabitliyor? Sen karar ver.
- Ajanlar: RepoTransAgent (Wang vd.), SWE-agent tarzı ajanlar (Khatry vd. Ek E), MigrationBench'in ajan sonuçları.
- Soru: hangileri gerçekten ajan? Sınıflandırmanı tabloya da yaz.

## 4. Çerçeveler, ürünler ve kıyaslama setleri (1,5)
- Kıyaslamalar: RepoTransBench, CRUST-Bench, MigrationBench (+ RustRepoTrans yedek). Ne ölçüyorlar: derlenme, test geçme, güvenli Rust, asgari/azami göç.
- Değerlendirme sorunu: Chen vd. 2025 — BLEU yetersiz; testler eksik olabilir → eşdeğerlik.
- Derleme: Liu vd. 2024 — ajan bileşenleri (planlama, bellek, algı, eylem).
- Özet tablo: Kaynak | LLM | Tasarım (RAG/FT/çerçeve) | Araçlar/eylemler | Test/kıyaslama | Sınıf | Ana bulgu (research/reading-notes.md sonunda).

## 5. Karşılaştırma ve boşluk (1,0) — birlikte tartışacağız
- Ajan ne zaman gerekçelendirilebilir? (depo düzeyi, derleyici/test geri bildirimi, belirsiz bağımlılık sırası…)
- Ne zaman iş akışı yeter? (tekrarlı, iyi tanımlı göçler; Google/Airbnb deseni)
- Eksik olan: davranış eşdeğerliğini doğrulama, test kapsamı, maliyet raporlama, güvenlik (testleri değiştirme, unsafe).

## 6. Demonun gerekçesi (0,5)
- Boşluğun hangi kısmı: "görünür testleri geçmek ≠ C ile aynı davranış" + ajanın ek döngüsünün maliyete değip değmediği.
- Model / talimat / araç değişikliğinin etkisi (bölüm 7'deki sonuçlardan).

## 7. Demo ve test sonuçları (2,5)
- Ajanın işlevi ve kurulumu: Claude Code (`claude -p`), araçlar Read/Write/Edit + yalnız `./run_tests.sh`; kişisel ayarlar kapalı; en çok 25 tur, en çok 10 test koşusu. Temel sistem: araçsız tek çağrı.
- Diyagram: docs/figures/agent-loop.svg (döngü, araçlar, onay adımı, izin katmanı).
- Test sonrası değişiklik: v1 → v1.1 (araç hatasında dolanma yasağı) ve etkisi; depo düzeyi vakaların eklenmesi.
- Test tablosu (Ek B'de tam: docs/appendix-b-tests.md), başarı oranı, ortalama adım ve maliyet; grafik: docs/figures/cost-vs-success.png.
- Eşdeğerlik: diferansiyel test (66 çeviri × ~400 girdi, C -O0/-O2 ile) — sözleşme içinde 0 fark; farklar yalnız C'nin tanımsız davranış bölgesinde (A1, H2). demo/results-summary.md son bölüm.
- 2–3 başarısız vaka + literatür bağlantısı (adaylar: S1/v2 testi onaysız değiştirme ↔ Anthropic 'guardrails'; A1/Haiku araç hatasını dolanma ↔ 'compounding errors' ve izin katmanı; A1 n=2147483647'de UB'nin sessizce düzeltilmesi ↔ eşdeğerlik boşluğu, Chen vd./CRUST-Bench'in test tabanlı başarı tanımı).
- Kanıt haritası: docs/evidence-map.md (iddia adayları, destek/karşı kanıt).
- Tür B: temel sistem vs ajan farkı; ek adım ve maliyete değdi mi?

## 8. Sonuç (0,25)

## Yapay zekâ kullanım beyanı (≤300 sözcük, sen yazarsın)
- Araçlar: Claude Code (Claude Opus/Sonnet/Haiku) — hangi işler için (kaynak arama, özet, demo kodu, dil düzeltme).
- Bir düzeltme/ret örneği: .ai/progress.md "AI output corrected/rejected" listesinden seç.

## Ekler (sayfa sınırına dahil değil)
- Ek A: arama kaydı (research/search-log.md)
- Ek B: tam test tablosu ve vaka tanımları (demo/cases.md, results)
- Ek C: seçilmiş adım/araç çağrısı kayıtları (demo/results/*/steps.txt)
- Ek D: diferansiyel test yöntemi ve özeti (demo/difftest.py, demo/difftest/report.json)
