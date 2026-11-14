# Okuma notları

Her kaynağı sen okuyup doldurursun / düzeltirsin. "(?)" = arama özetinden, okurken teyit et.
Alanlar yönergenin Yöntem bölümünden: temel model · RAG / ince ayar · çerçeve · araçlar · kıyaslama · bulgu · sınıf.
Sınıf testi (yönergenin 4 sorusu): S1 sonraki adıma model mi karar veriyor? S2 araç/dış veri? S3 kendini denetleyip döngü? S4 gerçek etkili eylem?

## 1. Anthropic (2024) Building effective agents — gri literatür
- Ana fikir: LLM uygulaması / iş akışı / ajan ayrımı; "en basit çözümle başla, karmaşıklığı ancak gerekince ekle".
- Raporda kullanım: sınıflandırma çerçevesi (Yöntem + Karşılaştırma).
- Senin notun:

## 2. Pan vd. (2024) Lost in Translation — ICSE 2024 — ampirik (ajan değil)
- Model: genel ve kod LLM'leri (?) · RAG/FT: yok (?) · Çerçeve: yok · Araçlar: yok · Kıyaslama: kendi veri seti, 5 dil (?)
- Bulgu: doğru çeviri %2,1–47,3 (?); 15 hata kategorisi (?); belirti tabanlı istem iyileştirmesi ~%5,5 (?)
- Sınıf: LLM uygulaması (tek istem) → temel sistemin literatür karşılığı.
- Senin notun:

## 3. Nikolov vd. (2025) How is Google using AI for internal code migrations? — endüstri / ampirik
- Model: Gemini tabanlı iç model (?) · Çerçeve: ortak göç araç seti · Araçlar: kod arama, derleme, test (?)
- Bulgu: "basit istem en basit göçler dışında yetmiyor" (?); JUnit3→4, Joda→java.time örnekleri.
- Sınıf: iş akışı mı ajan mı? (S1'e bak)
- Senin notun:

## 4. RepoTransBench (2024/2025) — kıyaslama + ajan
- Ölçek: v2 1.897 depo, 13 dil çifti (?) · Ajan: RepoTransAgent (ReAct) (?) · En iyi %32,8 (?)
- Hangi sürüme atıf: v1 mi v2 mi? Sabitle.
- Senin notun:

## 5. CRUST-Bench (2025) — kıyaslama + ajan, C→güvenli Rust
- 100 C deposu, elle yazılmış Rust arayüzü + testler (?) · tek atış en iyi: o1, 15 görev (?)
- Ek E: SWE-agent sonuçları — ajan basit üret-onar döngüsünü geçiyor mu? ARGÜMAN İÇİN KRİTİK, mutlaka oku.
- Senin notun:

## 6. MigrationBench (2025, AWS) — kıyaslama, sürüm yükseltme
- 5.102 / 300 depo; asgari vs azami göç (?) · SD-Feedback (?) · ajan çerçeveli sonraki sürüm sonuçları (?)
- Senin notun:

## 7. Airbnb (2025) Accelerating Large-Scale Test Migration with LLMs — gri literatür
- ~3.500 test dosyası; adım adım doğrulama kapıları + hata geri bildirimli yeniden deneme (?); kalan dosyalar elle (?)
- Sınıf: iş akışı (adımları kod sabitliyor) — teyit et.
- Senin notun:

## 8. Liu vd. (2024) LLM-Based Agents for Software Engineering: A Survey — derleme
- Ajan bileşenleri: planlama, bellek, algı, eylem (?) · çeviri/göç geçen kısımlar.
- Senin notun:

## 9. Chen vd. (2025) Systematic Literature Review on Neural Code Translation — derleme
- 57 çalışma, 2020–2025 (?) · BLEU yetersiz (?) · değerlendirme metrikleri bölümü.
- Senin notun:

## Özet tablo taslağı (rapor için; okuma bitince dolar)
| Kaynak | LLM | Tasarım (RAG/FT/çerçeve) | Araçlar/eylemler | Test/kıyaslama | Sınıf | Ana bulgu |
|---|---|---|---|---|---|---|
