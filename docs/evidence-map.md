# Kanıt haritası (tartışma için — argümanı sen kurarsın)

Her satır: tartışılabilir bir iddia adayı → destekleyen kanıt → karşı kanıt / sınırlılık. Kaynak sayfaları reading-notes.md'de.
Bunlar sonuç değil, tartışma girdisi. Hangisini savunacağına, nasıl tartacağına sen karar verirsin.

## A. Sınıflandırma: "ajan" denen başarıların çoğu iş akışı mı?
- Destek: Google — sabit akış (bul → düzenle → derle/test → onar → insan incelemesi), "agent-less techniques", planlama "often not needed" (Nikolov vd. s.10-11).
- Destek: Airbnb — durum makinesi, LLM yalnız başarısız adımı onarıyor (Covey-Brandt 2025).
- Karşı: MigrationBench ve RepoTransBench'te gerçek ajanlar (model sonraki adımı seçiyor) en iyi sonuçları veriyor.
- Nüans: CRUST-Bench "pipelined SWE-agent" melez — ilk adım sabit, onarım ajan. Sınır çizgisi keskin değil.
- Soru: endüstri (gerçek üretim) neden iş akışını, akademi (kıyaslama) neden ajanı öne çıkarıyor? (maliyet, öngörülebilirlik, inceleme darboğazı, değerlendirme ölçeği?)

## B. Ajan ne zaman değer katıyor?
- Destek: RepoTransBench — depo düzeyinde ajan %32,8, hata geri bildirimli iş akışı %11–16, tek atış %0.
- Destek: MigrationBench — azami göçte (bağımlılık yükseltme, kırılan API'ler) ajan + bilgi (PE, RAG) %15 → %53.
- Karşı: CRUST-Bench — ajan, basit test-onarım döngüsünü geçemedi (%32 = %32); adımların çoğu gezinme.
- Karşı: MigrationBench — hibrit (statik araç + ajan) aynı başarıyı daha az çağrıyla aldı.
- Demo: tek fonksiyon ve ~100–120 satırlık modül ölçeğinde ajan doğruluk eklemedi; ~2,2–2,4 kat maliyet.
- Aday sentez (senin kararın): değer, görevin belirsizliği ve keşif ihtiyacıyla (çok dosya, bilinmeyen bağımlılık, kırılan API) artıyor;
  iyi tanımlı, tekrarlı dönüşümlerde sabit iş akışı + doğrulama yetiyor.

## C. Güvenilirlik
- Kıyaslamalarda tavan düşük: RepoTransBench %32,8; CRUST-Bench %32 (en iyi ~%48, o3 test onarımı); MigrationBench azami %53.
- Endüstri: Google %87 değişmeden kabul — ama insan incelemesi ve sabit doğrulama sonrası; ölçüt zaman tasarrufu, doğruluk değil.
- Pan vd.: gerçek projelerde tek istem ≤%8,1 (2023 modelleri — zaman sınırlılığı).
- Demo: talimat kuralları kaldırılınca ajan testi kendisi değiştirdi (S1/v2); araç bozulunca Haiku 3/3 dolanmaya çalıştı, izin katmanı durdurdu.
- Demo: tek satır talimat (v1.1) dolanmayı 3/3 → 0/3 indirdi. Güvenilirlik = model + talimat + izin katmanı birlikte.

## D. Maliyet
- MigrationBench: başarı LLM çağrı bütçesiyle artıyor (33 → 59 çağrı); hibrit daha ucuz.
- Google: asıl maliyet/darboğaz insan incelemesi; değişiklik hızı bilerek sınırlandı.
- Anthropic: ajanlar "higher costs, and the potential for compounding errors".
- Demo: ajan tur 5–8, temel sistem 1; maliyet ~2,2–2,4 kat; doğrulukta fark yok.

## E. Boşluk: davranış eşdeğerliği nasıl doğrulanıyor?
- Literatür: hemen her yerde "testler geçiyor" = başarı (CRUST-Bench testleri, MigrationBench mvn verify, RepoTransBench test setleri).
  MigrationBench testi olmayan depoları dışarıda bırakıyor; kapsanmayan satırlarda verify = yalnız derleme.
- Chen vd.: alanın %42'si hâlâ BLEU kullanıyor; statik ölçütler yanıltıcı.
- Demo: gizli testler + 25.000 diferansiyel karşılaştırma → sözleşme içinde fark yok; farklar yalnız C'nin tanımsız davranış bölgesinde
  (A1: çeviriler UB'yi "düzeltti"; H2: Sonnet taklit etti, Haiku çöktü). "Davranışı koru" bu bölgede tanımsız.
- Aday boşluk ifadesi (senin kararın): eşdeğerliğin tanımı (özellikle UB ve sözleşme dışı girdi) ve test kapsamının ötesinde doğrulama
  (diferansiyel test, özellik tabanlı test) kıyaslamalarda yok; ajanların bunu kendiliğinden yaptığına dair kanıt da yok.

## F. Final projesine köprü (yönerge "Dönem Sonu Projesine Geçiş")
- Boşluk → ürün problemi: göç çevirisinin eşdeğerliğini testlerin ötesinde doğrulayan, belirsiz durumda insana soran bir araç.
- Test vakaları + kayıtlı adımlar → değerlendirme setinin başlangıcı (16 vaka, gizli testler, diferansiyel test).
- Temel sistem karşılaştırması → özerklik gerekçesi: tek modülde iş akışı yeter; çok dosyada ajan gerekebilir.
