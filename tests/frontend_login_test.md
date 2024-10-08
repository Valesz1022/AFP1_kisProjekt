# Bejelentkező felület tesztelési jegyzőkönyv
## 1.Áttekintés
**Projekt neve: Vicc-Explorer**
**Dátum: 2024. 10. 05. 21:20**
**Tesztelést végezte: Gunics Roland**
**Talált hibák száma: 1**

## 2. Tesztelési célok
- [A bejelentkező oldalon bejelentkezés elvégzése, különböző módon]
- [A bejelentkező oldalon a navigációs gombok ellenőrzése] 

## 3. Tesztelési környezet
**Operációs rendszer: Windows 10**
**Használt alkalmazás: Electron**

## 4. Bejelentkezés oldal funkcióinak tesztelése

### A bejelentkezés elvégzése különböző módokon
+ Létrehoztam két felhasználót, egy admin jogosultságút, és egy egyszerű felhasználó jogosultásgút.
+ Beírtam az egyszerű felhasználó jogosultságú profil felhasználó nevét és jelszavát, majd sikeresen bejelentkeztem.
+ Nem jelentek meg, csak azok a funkciók, amiket ezzel a jokgörrel kell látnom.
+ Visszamentem a bejelentkezés oldalra.
+ Bejelentkeztem sikeresen az admin jogosultságó felhasználó adataival.
+ Megjelentek a funkciók, amiket csak neki szabad látni.
+ Visszamentem a bejelentkezés oldalra.
+ Megpróbáltam bejelentkezni az admin jogosultságú felhasználóba, rossz felhasználó névvel.
+ Nem sikerült.
+ Megpróbáltam bejelentkezni az admin jogosultságú felhasználóba, rossz jelszóval.
+ Nem sikerült.
+ Megpróbáltam bejelentkezni nem létező adatokkal.
+ Nem sikerült.

### A bejelentkező oldalon a navigációs gombok ellenőrzése
+ Rákattintottam a Regisztráció gombra.
+ Megjelent a regisztrációs oldal helyesen.
+ Visszamentem a bejelentkezés oldalra.
+ Beaírtam valamit a felhasználónév szövegdobozba.
+ Rákattintottam a Bejelentkezés gombra.
+ Nem történt semmi, az adatok nem törlődtek a szövegdobozokból.