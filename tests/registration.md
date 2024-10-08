# Regisztráció felület tesztelési kézikönyv
## 1. Áttekintés
# Projekt neve: Vicc Explorer
# Dátum: 2024.10.08 16:32
# Tesztelést végezte: Pászti Szabolcs
# Talált hibák száma: 0

## 2. Tesztelési célok:
- Új felhasználó adatainak elküldése az adatbázisba
- A felületen lévő gombok ellenőrzése

## 3. Tesztelési környezet:
# Operációs rendszer: Windows 11
# Használt alkalmazás: Electron

## 4. Regisztráció oldal működésének ellenőrzése:

### Regisztráció ellenőrzése:
- Létrehoztam egy admint és egy átlag felhasználót.
- Sikeres regisztráció esetén consolra kiírtam hogy "Sikeres regisztráció".
- Adatbázisban lekérdeztem a benne lévő felhasználókat és megjelent az admin és az átlag felhasználó adatai.
- A név és a jelszó mezőt nem töltöttem ki ,úgy próbáltam meg regisztrálni megkaptam a hibaüzeneteket és nem lett elküldve az adatbázisba
- A név esetén megpróbáltam speciális karaktereket használni ,akkoris hibaüzeneteket kaptam.
- Létező felhasználó adataival se tudtam regisztrálni, 409-es hibakódot kaptam.

### Regisztráció oldalon lévő gombok ellenőrzése
- Rákattintottam a Bejelentkezés gombra, megjelent a Bejelentkezési oldal.
- Vissza kattintottam a Regisztráció gombra visszakerültem a regisztrációs felületre.
- Megfelelő input megadások után a Regisztráció gombra kattintva bekerült az adatbázisba az új felhasználó.
- Szövegdobozokba kattintva tudtam bele gépelni.