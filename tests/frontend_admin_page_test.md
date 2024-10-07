# Főoldal, admin szemszögéből, tesztelési jegyzőkönyv
## 1.Áttekintés
**Projekt neve: Vicc-Explorer**
**Dátum: 2024. 10. 05. 21:48**
**Tesztelést végezte: Gunics Roland**
**Talált hibák száma: 2**

## 2. Tesztelési célok
- A főoldalon a szavazás funkció tesztelése
- A főoldalon a törlés funkció tesztelése
- A főoldalon a navigációs gombok ellenőrzése

## 3. Tesztelési környezet
**Operációs rendszer: Windows 10**
**Használt alkalmazás: Electron**

## 4. Főoldal, admin szemszögéből, funkcióinak tesztelése

### A főoldalon a szavazás funkció tesztelése
+ Hozzáadtam a főoldalhoz egy új viccet.
+ Rákattintottam az upvote gombra.
+ A gomb beszineződott és a számláló frissült.
+ Rákattintottam mégegyszer.
+ A gomb visszafeketedett és a számláló frissült.
+ Rákattintottam mégegyszer.
+ Rákattintottam a downvote gombra.
+ Nem történt semmi.
+ Rákattintottam az upvote gombra.
+ Ez jól működött megint.
+ Rákattintottam a downvote gombra.
+ Így beszineződött és a számláló frissült.
+ Rákattintottam megint.
+ A gomb visszafeketedett és a számláló frissült.
+ Rákattintottam megint.
+ Rákattintottam az upvote gombra.
+ Nem történt semmi.

### A főoldalon a törlés funkció tesztelése
+ Hozzáadtam a főoldalhoz még kettő viccet, így 3 volt.
+ Helyesen jelentek meg egymás alatt, jól elkülöníthetően.
+ Rákattintottam a törlés gombra, a középső viccnél.
+ A vicc törlődött, az alatta lévő felcsúszott a helyére.
+ Megismételtem az első vicc esetében is.
+ Ismét jól működött.
+ Elvégeztem a 3. viccnél is.
+ Ez is jól működött

### A főoldal navigációs gombjainak a használata
+ Rákattintottam a vicc hozzáadása gombra.
+ Átnavigált a megfelelő helyre.
+ Onnan sikeresen vissza tudtam jönni.
+ Rákattintottam a Kijelentkezés gombra.
+ Sikeresen kijelentkeztem.
+ Bejelentkeztem újra.
+ Rákattintottam a Profil gombra.
+ Várhatóan nem történt semmi.