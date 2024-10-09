# Főoldal, átlag felhosználó szemszögéből, tesztelési jegyzőkönyv
## 1.Áttekintés
**Projekt neve: Vicc-Explorer**
**Dátum: 2024. 10. 08. 20:32**
**Tesztelést végezte: Pászti Szabolcs**
**Talált hibák száma: 2**

## 2.Tesztelési célok
- Az oldalon lévő navigációs gombok tesztelése.
- Az oldalon lévő szavazások tesztelése.

## 3. Tesztelési környezet
**Operációs rendszer: Windows 11**
**Használt alkalmazás: Electron**

## 4. Főoldal, átlag felhasználó szemszögéből, funkcióinak tesztelése

### Az oldal szavazás gombjainak a tesztelése
- Létrehoztam egy új viccet.
- Rákattintottam az upvote gombra.
- A gomb beszineződott és a számláló frissült.
- Rákattintottam mégegyszer.
- A gomb visszafeketedett és a számláló frissült.
- Rákattintottam mégegyszer.
- Rákattintottam a downvote gombra.
- Nem történt semmi.
- Rákattintottam az upvote gombra.
- Ez jól működött megint.
- Rákattintottam a downvote gombra.
- Így beszineződött és a számláló frissült.
- Rákattintottam megint.
- A gomb visszafeketedett és a számláló frissült.
- Rákattintottam megint.
- Rákattintottam az upvote gombra.
- Nem történt semmi.

### Az oldal navigációs gombjainak tesztelése
- Rákattintottam az új vicc gombra.
- Átnavigált a megfelelő oldalra.
- Rákattintottam a kijelentkezés gombra.
- Kijelentkeztetett és átvitt a bejelentkezés oldalra.
- Bejelentkeztem újra és visszakerültem a főoldalra.
- Rákattintottam a Profil gombra.
- Nem csinált semmit