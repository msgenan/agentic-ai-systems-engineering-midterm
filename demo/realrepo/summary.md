# Gercek proje deneyi: minishell ayristirma yolu (C -> Rust)

Tarih: 2026-11-15. Model: sonnet. Her konfigurasyon 3 tekrar (sure yetti, azaltmaya gerek olmadi).

## Birim
Kaynak: `msgenan/42Cursus-minishell` (commit 2916f01, yerel klon `external/minishell/`, degistirilmedi).
Secilen alt sistem: lexer + soz dizimi denetimi + parser + tirnak/degisken genisletme (satir -> token/komut yapisi).
Modellere verilen C: 889 satir (14 .c dosyasi: lexer/, parser/, parser/expand/, utils/add/, utils/syntax.c,
utils/tools/quote_tools.c) + gerekli yardimci dosyalar (init.c, environment.c, exit_error.c, free_funcs.c, 8 libft dosyasi).
Tum depo: ~3970 satir C (libft 1261, utils 1145, executor 519, builtins 470, parser 346, lexer 160).
readline bagimliligi: birimin disinda kaldi (surucu readline cagirmiyor; derleme sirasinda yalnizca baslik gerekiyor).

## Duzen
- `cdriver/driver.c`: C surucu (main.c `process()` ile ayni akis, executor yok; stdin satir -> kanonik dokum). Minishell mantigi degismedi; yalniz
  `set_get_shell`, `signal_handler` ve 3 hata isleyici icin kod-yapistirma (stub) var. `$?` = 7, sabit env (kopya anahtar, `NOVAL`, `EMPTY`, `EQ=a=b` dahil).
- `template/` + `cases/minishell_parse/`: Rust crate (`stub.rs` = `todo!()` API), Rust surucu `src/bin/driver.rs` (ayni dokum), `visible.rs` (12 test,
  beklenen degerler C suruculerden uretildi: `make_visible.py`), `hidden_inputs.txt` (518 girdi: 120 elle yazilmis + tohumlu rastgele/yapisal,
  `gen_inputs.py`, seed 20261115), `reference.rs` (dogrulama icin elle yazilmis sadik ceviri).
- `run_realrepo.py`: `demo/run.py`'deki `run_baseline`/`run_agent` ve temiz `claude -p` bayraklarini yeniden kullanir (agent: max-turns 40, zaman asimi 20 dk).
  Degerlendirme: orijinal test/surucu/Cargo.toml geri yuklenir, her uygulama icin ayri `CARGO_TARGET_DIR`, visible testler + 518 girdide C ile Rust
  ciktisi birebir karsilastirilir (satir basina ayri surec). Kural ihlalleri: test/surucu/run_tests.sh degisimi, `unsafe`, Cargo.toml degisimi.
- Dogrulama: `reference.rs` 518 girdide 0 uyumsuzluk. (Gelistirme sirasinda referansin ilk surumu `export` kontrolunu orijinal argv[0] uzerinden
  yaptigi icin 1/518 uyumsuzluk verdi: C kodu argv[0]'i yerinde genisletip sonra kiyasliyor. Fark yakalandi, yani test duyarli.)

## Sonuclar
| Config | Calisma | Visible | Uyumsuzluk (518) | Tur | Arac cagrisi | Maliyet ($) | Sure (s) | Ihlal |
|---|---|---|---|---|---|---|---|---|
| baseline-sonnet | r1 | gecti | 0 | 1 | - | 0.181 | 37 | yok |
| baseline-sonnet | r2 | gecti | 0 | 1 | - | 0.072 | 43 | yok |
| baseline-sonnet | r3 | gecti | 0 | 1 | - | 0.070 | 41 | yok |
| agent-sonnet-v1 | r1 | gecti | 0 | 15 | 14 (13 Bash, 1 Write) | 0.228 | 62 | yok |
| agent-sonnet-v1 | r2 | gecti | 0 | 22 | 21 | 0.255 | 70 | yok |
| agent-sonnet-v1 | r3 | gecti | 0 | 12 | 11 | 0.279 | 63 | yok |

Ozet: baseline 3/3, agent 3/3 tam basari; tavan etkisi 889 satirlik gercek kod ile de kirilmadi. Agent ortalama ~3.5x daha pahali
(0.25$ vs 0.11$) ve ~1.6x daha yavas; bu birimde dogruluk farki yok. Ajan C'yi calistiramadi (araci yok), yalniz gorunur testlerle dogruladi;
fark kapanmasi bu yuzden C kaynagini dikkatli okumaktan geliyor, arac dongusunden degil.

## Basarisizlik ornegi
Yok: 6 calismanin hicbirinde uyumsuzluk cikmadi. Tek "yakalanan" hata deneyin kendi referans cevirimdeki `export` ornegi (yukarida):
girdi `export'' $NOSUCH ...`; C: `"export","$NOSUCH",...` (argv[0] yerinde sadelesince `export` dali devreye girer, argumanlar genisletilmez),
hatali Rust: `"export","","/to/f",...`.

## Sinirliliklar
- Hidden girdi kumesi benim yazdigim bir uretecten; modellerin hic gormedigi ama benim bildigim tuzaklar (tek tirnak oneki $ genisletmeyi kapatir,
  `$e-acute` ASCII-only, `export` yerinde kiyas, `$$`, ilk anahtar kazanir) kapsaniyor, yine de rastgele girdilerin sinirli bir ornegi.
- Satir sayisi 889: "1500'e kadar" araliginin altinda; modellerin tek seferde ve bellek icinde tutabilecegi boyut. Tavani kirmak icin daha buyuk
  (ornegin executor/heredoc dahil, durumlu) bir birim ya da zorlastirilmis gorunur test ayrimi gerekir.
- Girdiler UTF-8 gecerli, NUL yok; C tarafi ASAN/UBSAN altinda temiz (leak kontrolu kapali).
- Tek model (sonnet), tek prompt (v1), n=3.
