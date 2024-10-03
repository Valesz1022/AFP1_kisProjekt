# Felhasználók
## Regisztráció
### Kérés
Végpont: /register?name=&password=\
Metódus: POST
### Válaszok
#### 200 Ok
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
#### 200 Ok
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
