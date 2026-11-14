# Aday kaynaklar

Durum: arama sonucu düzeyinde doğrulandı (künye, arXiv/DOI var). Rakamlar sürümler arasında değişiyor; atıfta
kullanılan sürüm okunurken sabitlenecek. Sınıf = sistemin gerçekte ne olduğu (LLM uyg. / iş akışı / ajan), okurken teyit.

## Derleme
1. Liu, J. vd. (2024). Large Language Model-Based Agents for Software Engineering: A Survey. arXiv:2409.02977.
   Genel SE ajan derlemesi (planlama, bellek, algı, eylem). Okuma: giriş + ajan çerçevesi + "code translation/migration" geçen kısımlar.
2. Chen, X. vd. (2025). A Systematic Literature Review on Neural Code Translation. arXiv:2505.07425.
   Alan derlemesi, 57 çalışma; BLEU'nun yetersizliği. Okuma: değerlendirme metrikleri ve açık sorunlar.

## Ampirik
3. Pan, R. vd. (2024). Lost in Translation: A Study of Bugs Introduced by LLMs while Translating Code. ICSE 2024. doi:10.1145/3597503.3639226
   Tek istemli çeviri (LLM uygulaması): doğru çeviri %2,1-47,3; 15 hata kategorisi. Temel sistemin literatür karşılığı.
4. Ibrahimzada, A. R. vd. (2025). AlphaTrans: Repository-Level Compositional Code Translation and Validation. FSE 2025. doi:10.1145/3729379
   Java->Python, program analizi + LLM, parça parça çevir ve testle doğrula. Sınıf: büyük olasılıkla iş akışı.
5. Nikolov, S. vd. (2025). How is Google using AI for internal code migrations? arXiv:2501.06972.
   Endüstri deneyim raporu: "basit istem en basit göç dışında yetmiyor"; JUnit3->4, Joda->java.time. Sınıf: iş akışı.
6. Google (2025). Migrating Code At Scale With LLMs At Google. arXiv:2504.09691. (yazarlar okunurken eklenecek)
   12 ayda 3 geliştirici, 39 göç. 5 ile birlikte ya da onun yerine.
7. RepoTransBench (2024/2025). arXiv:2412.17744. (yazarlar eklenecek)
   v2: 1.897 depo, 13 dil çifti; ReAct tabanlı RepoTransAgent, en iyi yöntem %32,8. Hem kıyaslama hem ajan sonucu.
8. CRUST-Bench (2025). C-to-safe-Rust Transpilation Benchmark. arXiv:2504.15254. (yazarlar eklenecek)
   100 C deposu, el yazımı güvenli Rust arayüzü + testler; tek atışta en iyi model 15 görev. Ek E: SWE-agent sonuçları
   (ajan vs basit üret-onar döngüsü iddiası okunarak doğrulanacak). Demo ile birebir örtüşen kaynak.

## Kıyaslama
9. MigrationBench (2025). Repository-Level Code Migration Benchmark from Java 8. arXiv:2505.09569. (AWS AI Labs)
   5.102 depo; asgari/azami göç; SD-Feedback; sonraki sürümde ajan çerçevesi. Sürüm yükseltme kolu.
10. RustRepoTrans (2025). Repository-level Code Translation Benchmark Targeting Rust. ASE 2025, arXiv:2411.13990.
   375 görev, C/Java/Python->Rust; depo düzeyinde başarı ciddi düşüyor. (7-9 yetiyorsa yedek)

## Gri literatür
11. Covey-Brandt, C. (2025). Accelerating Large-Scale Test Migration with LLMs. Airbnb Tech Blog. https://airbnb.tech/?p=366
   ~3.500 test dosyası, adım adım doğrulama kapıları + hata geri bildirimli yeniden deneme. Sınıf: iş akışı. Rakamlar özgün yazıdan doğrulanacak.
12. Anthropic (2024). Building effective agents. https://www.anthropic.com/research/building-effective-agents
   Sınıflandırma çerçevesinin kaynağı (yönerge bunu esas alıyor).
13. DARPA (2024). Translating All C to Rust (TRACTOR). https://www.darpa.mil/research/programs/translating-all-c-to-rust
   Politika: neden C->Rust önemli (bellek güvenliği). Yedek.

## Önerilen seçim (9)
1, 2 (derleme) · 3, 5, 7, 8 (ampirik) · 9 (kıyaslama) · 11, 12 (gri).
Dağılım kontrolü: derleme ≥1 (2), ampirik ≥3 (ajanı test eden: 7, 8, 9'un ajan bölümü; 3 ve 5 karşılaştırma için), kıyaslama ≥1, gri ≥1.
Açık nokta: "ajanı test eden ampirik" şartı için 3 ve 5 ajan değil; ajan sonucu raporlayanlar 7, 8 ve 9. Okurken bu teyit edilmeli.
