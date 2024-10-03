# Felhasználók
## Regisztráció
### Kérés
Végpont: /register?name=&password=\ Metódus: POST
### Válaszok
#### 201 Created (elkészítve)
Jelentés: Sikeres regisztráció\
Törzs: nincs 
#### 409 Conflict (foglalt)
Jelentés: Sikertelen regisztráció, létezik már a felhasználó\
Törzs: Hibaüzenet.
#### 422 Unprocessable Entity (feldolgozhatatlan egység)
Jelentés: Nem sikerült regisztrálni, valamelyik paraméter nem volt megadva.\
Törzs: nincs
## Bejelentkezés
### Kérés
Végpont: /login?name=&password=\
Metódus: POST
### Válaszok
#### 200 Ok
Jelentés: Sikeres bejelentkezés.\
Törzs: A felhasználó neve, illetve admin jogosultsága van-e.
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Sikertelen bejelentkezés valamilyen hiba miatt 
(pl. nem helyes adatok lettek megadva)\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Sikertelen bejelentkezés szerveroldali hiba miatt.\
Törzs: nincs
## Kijelentkezés
### Kérés
Végpont: /logout\
Metódus: GET
### Válaszok
#### 200 Ok
Jelentés: Sikeres kijelentkezés.\
Törzs: nincs
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Sikertelen bejelentkezés, nincsen bejelentkezve a felhasználó.\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Sikertelen kijelentkezés, valamilyen szerveroldali hiba miatt.\
Törzs: nincs.
# Viccek
## Lekérdezés
### Kérés
Végpont: /jokes\
Metódus: GET
### Válasz
#### 200 Ok
Jelentés: Sikeres lekérdezés.\
Törzs: A feltöltött viccek egy tömbben.
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Valami hiba történt.\
Törzs: Hibaüzenet.
## Feltöltés
### Kérés
Végpont: /jokes?name=&content=\
Metódus: POST\
Előtte be kell jelentkezni!
### Válasz
#### 201 Created (elkészítve)
Jelentés: Sikeres feltöltés.\
Törzs: nincs.
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Nincs bejelentkezve felhasználó.\
Törzs: nincs
#### 409 Conflict (foglalt)
Jelentés: Már van ilyen vicc ettől a felhasználótól.\
Törzs: Hibaüzenet.
#### 422 Unprocessable Entity (feldolgozhatatlan egység)
Jelentés: Hibás kérés paraméterek.\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Valami hiba történt a szerveren.\
Törzs: nincs
## Törlés
### Kérés
Végpont: /jokes?id=\
Metódus: DELETE\
Előtte be kell jelentkezni egy admin jogosultságú felhaszálóval!
### Válaszok
#### 200 Ok
Jelentés: Sikeres törlés.\
Törzs: nincs
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Nincs bejelentkezve felhasználó.\
Törzs: nincs
#### 403 Forbidden (hozzáférés megtagadva)
Jelentés: A bejelentkezett felhasználó nem admin jogosultságú.\
Törzs: nincs
#### 404 Not Found (nem található)
Jelentés: Nincs ilyen azonosító az adatbázisban.\
Törzs: Hibaüzenet.
#### 422 Unprocessable Entity (feldolgozhatatlan egység)
Jelentés: Hibás kérés paraméterek.\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Valami hiba történt a szerveren.\
Törzs: nincs
# Szavazás (upvote, downvote)
## Új szavazat leadása
### Kérés
Végpont: /votes?name=&joke_id=&vote\
Metódus: POST\
Előtte be kell jelentkezni!\
Használat: vote-nak -1-nek kell lennie downvote-hoz, 1-nek upvote-hoz.
### Válaszok
#### 201 Created (elkészítve)
Jelentés: Sikeres szavazás.\
Törzs: nincs
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Nincs bejelentkezve felhasználó.\
Törzs: nincs
#### 404 Not Found (nem található)
Jelentés: Nincs ilyen azonosító az adatbázisban.\
Törzs: Hibaüzenet.
#### 422 Unprocessable Entity (feldolgozhatatlan egység)
Jelentés: Hibás kérés paraméterek.\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Valami hiba történt a szerveren.\
Törzs: nincs
## Meglévő szavazat módosítása
### Kérés
Végpont: /votes?name=&joke_id=&vote=\
Metódus: PUT\
Előtte be kell jelentkezni!\
Használat: vote-nak -1-nek kell lennie downvote-hoz, 1-nek upvote-hoz.
### Válaszok
#### 200 Ok
Jelentés: Sikeres módosítás.\
Törzs: nincs
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Nincs bejelentkezve felhasználó.\
Törzs: nincs
#### 409 Conflict (foglalt)
Jelentés: Nincs ilyen azonosító az adatbázisban, vagy erre még nem szavazott
a felhasználó.\
Törzs: Hibaüzenet.
#### 422 Unprocessable Entity (feldolgozhatatlan egység)
Jelentés: Hibás kérés paraméterek.\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Valami hiba történt a szerveren.\
Törzs: nincs
## Meglévő szavazat törlése
### Kérés
Végpont: /votes?name=&joke_id=\
Metódus: DELETE\
Előtte be kell jelentkezni!
### Válaszok
#### 200 Ok
Jelentés: Sikeres módosítás.\
Törzs: nincs
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Nincs bejelentkezve felhasználó.\
Törzs: nincs
#### 409 Conflict (foglalt)
Jelentés: Nincs ilyen azonosító az adatbázisban, vagy erre még nem szavazott
a felhasználó.\
Törzs: Hibaüzenet.
#### 422 Unprocessable Entity (feldolgozhatatlan egység)
Jelentés: Hibás kérés paraméterek.\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Valami hiba történt a szerveren.\
Törzs: nincs
# Elmentett viccek
## Felhasználó elmentett vicceinek lekérdezése
### Kérés
Végpont: /saved?name=\
Metódus: GET\
Előtte be kell jelentkezni!
### Válaszok
#### 200 Ok
Jelentés: Sikeres lekérdezés.\
Törzs: Az adott felhasználó által elmentett viccek.
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Nincs bejelentkezve felhasználó.\
Törzs: nincs
#### 404 Not Found (nem található)
Jelentés: Nincs ilyen felhasználó az adatbázisban, vagy még nem mentett
el semmit.\
Törzs: Hibaüzenet.
#### 422 Unprocessable Entity (feldolgozhatatlan egység)
Jelentés: Hibás kérés paraméterek.\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Valami hiba történt a szerveren.\
Törzs: nincs
## Új vicc elmentése
### Kérés
Végpont: /saved?name=&joke_id=\
Metódus: POST\
Előtte be kell jelentkezni!
### Válaszok
#### 201 Created (elkészítve) 
Jelentés: Sikeres módosítás.\
Törzs: nincs
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Nincs bejelentkezve felhasználó.\
Törzs: nincs
#### 404 Not Found
Jelentés: Nincs ilyen felhasználó vagy poszt az adatbázisban.\
Törzs: Hibaüzenet.
#### 422 Unprocessable Entity (feldolgozhatatlan egység)
Jelentés: Hibás kérés paraméterek.\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Valami hiba történt a szerveren.\
Törzs: nincs
## Mentett vicc törlése mentések közül
### Kérés
Végpont: /saved?name=&joke_id=\
Metódus: DELETE\
Előtte be kell jelentkezni!
### Válaszok
#### 200 Ok
Jelentés: Sikeres törlés.\
Törzs: nincs
#### 401 Unauthorized (jogosulatlan hozzáférés)
Jelentés: Nincs bejelentkezve felhasználó.\
Törzs: nincs
#### 404 Not Found
Jelentés: Nincs ilyen felhasználó vagy poszt az adatbázisban, vagy nincs neki
ez a poszt elmentve.\
Törzs: Hibaüzenet.
#### 422 Unprocessable Entity (feldolgozhatatlan egység)
Jelentés: Hibás kérés paraméterek.\
Törzs: nincs
#### 500 Internal Server Error (belső szerverhiba)
Jelentés: Valami hiba történt a szerveren.\
Törzs: nincs
# Állapot ellenőrzése
## Kérés
Végpont: /health_check\
Metódus: GET
## Válasz
### 200 Ok
Jelentés: Fut a szerver.\
Törzs: {"status": "active"}
