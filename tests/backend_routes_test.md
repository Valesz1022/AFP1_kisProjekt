# Backend végpontok tesztelése
Tesztelést végezte: Zana Domán\
Operációs Rendszer: Arch Linux x86_64 (Linux 6.11.2-arch1-1)\
Böngésző: Google Chrome Stable\
Dátum: 2024.10.08.\
Talált hibák száma: 1

# Vicc kezelés tesztelése
- Lekérdezés: a szerver sikeresen visszadja az összes létező viccet az adatbázisból.
- Feltöltés: a szerver sikeresen vizsgálja, hogy meg lett-e adva a felhasználó neve\
  és a vicc tartalma, ha ezek nincsenek meg akkor 422-es hibával jelzi.\
  Amennyiben a felhasználó nem létezik vagy a vicc tartalma már megtalálható az\
  adatbázisban akkor a szerver 409-es hibával jelzi a kérés sikertelenségét.\
  Helyes felhasználónév és új tartalom esetén a hozzáadás sikeres, 201-es\
  kóddal jelzi.
- Törlés: a szerver sikeresen vizsgálja, hogy meg lett-e adva a vicc azonosítója,\
  ha nem lett akkor 422-es státusszal jelzi. Amennyiben a törölni kívánt vicc nem\
  létezik a szerver 404-es hibát küld vissza, ellenkező esetben a viccet sikeresen\
  törli és 200-as kódot küld.

# Értékelési funkciók tesztelése
- Lekérdezés: a szerver ellenőrzi, hogy meg lett-e adva az értékelést adó felhasználó\
  és az értékelt vicc azonosítója, ha nem akkor 422-es hibát ad vissza. Ellenkező\
  esetben amennyiben létezik vissza adja az értékét 1-el vagy -1-el jelezve az\
  értékelés típusát, 0-val pedig azt, hogy az adott vicc még nem volt értékelve\
  az adott felhasználó által.
- Értékelés adása: a szerver ellenőrzi, hogy a felhasználó neve, a vicc azonosítója\
  és az értékelés típusa meg lett-e adva, ha nem akkor 422-es hibával jelzi.\
  Ha az adott viccet az adott felhasználó már értékelte akkor 409-es hibával jelzi,\
  amennyiben nincs hiba az értékelés sikeresen elmentésre került és 201-as kódot ad.
- Értékelés frissítése: a szerver ellenőrzi, hogy a felhasználó neve, a vicc\
  azonosítója és az értékelés típusa meg lett-e adva, ha nem akkor 422-es hibával\
  jelzi. Ha a viccet a felhasználó még nem értékelte akkor a szerver 404-es kóddal\
  jelzi a sikertelenséget. Ha minden rendben van a kéréssel és az értékelés már\
  létezik a frissítendő értékelés, akkor sikeresen elmentésre kerül az új értékelés\
  és a szerver 200-as kódot küld vissza.
- Értékelés törlése: a szerver ellenőrzi, hogy meg lett-e adva a felhasználó neve\
  és a vicc azonosítója, ha nem akkor 422-es kóddal jelzi. Ha az adott felhasználó\
  értékelés pár nem létezik, akkor a szerver 409-es hibát küld. Ha minden rendben volt\
  akkor a szerver sikeresen törli az adott értékelést és 200-as kódot ad vissza.

# Mentési funckciók tesztelése
- Lekérdezés: a szerver lekérdezi, hogy megadtuk-e a felhasználó nevét, ha nem\
  akkor 422-es kóddal jezi a hibát. Ha a felhasználónak nincs mentett vicce\
  akkor 404-el jelzi. Ha a kéréssel minden rendben volt és a felhaszáló mentett\
  már el viccet, akkor az összes mentett viccet vissza küldi 200-as kóddal.
- Mentés: a szerver ellenőrzi, hogy meg lett-e adva felhasználó neve és a menteni\
  kívánt vicc azonosítója, ha nem akkor 422-es kóddal jelzi a hibát. Hiba: ha\
  a menteni kívánt viccet már elmentette a felhasználó korábban, akkor nem 409-es,\
  hanem 500-as hibát ad vissza a szerver ezzel hamisan azt jelezve, hogy belső\
  szerver hiba történt helytelen kérés helyett. Ha a kérés helyes akkor a mentés\
  sikeresen mentésre kerül és 201-es kódot kapunk vissza.
- Törlés: a szerver ellenőrzi, hogy megadásra került-e a felhasználó neve és\
  a vicc azonoítója, amennyiben nem akkor a szerver 422-es hibakóddal jelzi.\
  Ha a felhasználó nem mentette el az adott viccet akkor a szerver 409-es\
  kóddal jelzi. Ha minden rendben volt a kéréssel akkor a szerver sikeresen\
  törli a mentést és 200-as kódot ad vissza.
