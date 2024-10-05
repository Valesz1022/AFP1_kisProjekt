# Backend hitelesítés és jogosultságok tesztelése
Tesztelést végezte: Molnár Máté Norbert\
Operációs Rendszer: Arch Linux 6.10.10\
Böngésző: Firefox\
Dátum: 2024. 10. 05.\
Talált hibák száma: 1

# Hitelesítése funkciók tesztelése
- Elindítottam a szervert, illetve az adatbázist.
- Megpróbáltam feltölteni egy új viccet bejelentkezés nélkül, a szerver
401 Unauthorized státuszt adott vissza, amely a megfelelő.
- Regisztráltam egy új fiókkal.
- Megpróbáltam bejelentkezni a fiókba, rossz adatokat megadva, szintén 401
Unauthorized státuszkódot kaptam, amely megfelelő, hiszen nem létezik olyan fiók
amelynek az adatait megadtam.
- Megpróbáltam újra bejelentkezni a fiókba, immáron jó adatokat megadva, és
sikerült, jelezve a 200 Ok státuszkód alapján.
- Megpróbáltam feltölteni egy új viccet, jó adatokat megadva, és a szerver 201
Created státuszkódot küldött vissza, jelezve, hogy sikeresen feltöltöttem az új
viccet.
- Ellenőriztem, hogy ténylegesen sikeres volt-e a feltöltés, lekérdeztem az
összes viccet, 200 Ok státuszkóddal válaszolt, illetve a válasz törzsében
megtaláltam legutolsó pozícióban az újonnan feltöltött viccemet.

## Kijelentkezés
- Bejelentkeztem az újonnan létrehozott fiókba.
- Feltöltöttem egy új viccet.
- Megpróbáltam kijelentkezni a fiókból, a szerver 200 Ok státuszkóddal jelezte,
hogy sikerült kijelentkeznem.
- Megpróbáltam újonnan feltölteni egy viccet, jó adatokat megadva, a szerver
401 Unauthorized státuszkóddal jelezte, hogy nem vagyok bejelentkezve, ezért
nincsen jogosultságom feltölteni, tehát sikeresen kijelentkeztem.

# Jogosultságok tesztelése
- Bejelentkeztem az előbb létrehozott fiókba, amely "felhasználó" jogosultságot
kapott, mivel minden újonnan létrehozott felhasználó azt kap.
- Feltöltöttem egy új viccet.
- Megpróbáltam törölni a viccet, a jó adatokat megadva, viszont a szerver
403 Forbidden státuszkóddal válaszolt, amely a megfelelő, hiszen csak admin
jogosultságú felhasználók törölhetnek vicceket.
- Kijelentkeztem a felhasználóból sikeresen.
- Bejelentkeztem egy admin jogosultságú fiókba.
- Megpróbáltam így törölni egy viccet, először rossz adatokat megadva, a szerver
422 Unprocessable Entity státuszkóddal válaszolt, jelezve, hogy valamit rosszul
adtam meg.
- Megpróbáltam törölni ismételten egy viccet, mostmár a jó adatokat megadva,
a szerver 200 Ok státuszkóddal válaszolt, amelyből sejtettem, hogy sikeres volt
a törlés.
- Ellenőriztem, hogy ténylegesen sikeres volt-e a törlés, lekérdeztem az összes
feltöltött viccet, és valóban nem találtam meg a kitörölt viccet.
- Kijelentkeztem az admin jogosultságú fiókból.
- Bejelentkeztem a tesztelések elején létrehozott felhasználói jogosultságú
fiókba.
- Lekérdeztem ismételten a feltöltött vicceket, és így sem találtam meg benne 
amit az admin fiókról töröltem.

## Jogosultságok módosításának tesztelése
- Bejelentkeztem a tesztelés elején létrehozott felhasználói fiókba.
- Feltöltöttem a szerverre kettő új viccet, ellenőriztem, hogy sikeres volt-e a
feltöltés.
- Megpróbáltam törölni az egyik újonnan feltöltött viccet, a szerver 403 
Forbidden státuszkóddal jelezte, hogy nincsen jogosultságom.
- Az adatbázison keresztül átírtam jogosultságomat admin-ra.
- Megpróbáltam ismételten törölni az egyik viccet, a szerver 200 Ok 
státuszkóddal jelezte, hogy van már jogosultságom, és sikeres volt a törlés.
- Ellenőriztem, ténylegesen sikeres volt.
- Visszaállítottam az újonnan létrehozott fiók jogosultságát.
- Megpróbáltam törölni a másik viccet, viszont ismételten nem sikerült, a 
szerver 403 Forbidden státuszkóddal válaszolt, jelezve, hogy ismételten
nincsen jogosultságom.
