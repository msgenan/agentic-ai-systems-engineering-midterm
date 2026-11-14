# Arama kaydı (ek için)

Tarih: 14 Kasım 2026. Araç: web araması (Claude Code WebSearch), sonuçlar arXiv, ACM, kurum siteleri üzerinden açıldı.
Not: yönerge en az iki veri tabanı istiyor; aşağıdaki sorgular Google Scholar ve Semantic Scholar'da da
çalıştırılıp sonuç sayıları eklenecek (sahip okurken).

| # | Sorgu | Hedef | Bulunan aday |
|---|-------|-------|--------------|
| 1 | "How is Google using AI for internal code migrations" | endüstri deneyim raporu | Nikolov vd. 2025; Google 2025 (39 göç) |
| 2 | "Lost in Translation" LLM code translation bugs ICSE 2024 | ampirik, ajan olmayan temel | Pan vd. 2024 |
| 3 | RepoTransBench repository-level code translation benchmark | kıyaslama + ajan | RepoTransBench (+ RepoTransAgent) |
| 4 | MigrationBench repository-level code migration Java 8 | kıyaslama (sürüm yükseltme) | MigrationBench (AWS) |
| 5 | Airbnb LLM test migration Enzyme React Testing Library | gri literatür | Airbnb 2025 blog |
| 6 | LLM agent C to Rust translation repository-level empirical | ampirik, C->Rust | CRUST-Bench, RustRepoTrans, ORBIT, DepTrans, SmartC2Rust |
| 7 | CRUST-Bench C-to-safe-Rust SWE-agent | ajan vs basit döngü | CRUST-Bench Ek E |
| 8 | survey LLM-based agents for software engineering | derleme (genel) | Liu vd. 2024 |
| 9 | survey LLM code translation migration review | derleme (alan) | Chen vd. 2025 SLR |
| 10 | DARPA TRACTOR Translating All C to Rust | politika / gri | DARPA 2024 |
| 11 | AlphaTrans repository-level code translation | ampirik, iş akışı | Ibrahimzada vd. 2025 |

Dahil etme ölçütleri (taslak): 2023 ve sonrası; LLM tabanlı kod çevirisi veya göçü; sonuç raporlayan
(başarı oranı, maliyet ya da başarısızlık analizi); hakemli yayın, arXiv ön baskısı veya birincil kurum kaynağı.
Hariç: yalnız BLEU gibi metin benzerliği raporlayan, LLM öncesi (TransCoder vb.) çalışmalar.
