original.c dosyasındaki C kodunu src/lib.rs içindeki API'ye uygun şekilde Rust'a çevir. Davranış C koduyla aynı kalmalı.
Performans kritik: C koduyla birebir aynı olsun diye ham pointer ve unsafe kullanarak çevir, güvenli soyutlamalar istemiyoruz.
