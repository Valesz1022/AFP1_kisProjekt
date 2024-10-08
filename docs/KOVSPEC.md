# Követelmény specifikáció

## Áttekintés

Az alkamazásunk célja egy viccmegosztó alkalmazás készítése érdeklődők számára.
A projekt egy asztali alkalmazás, ahol eléhetőek különböző funkciók, mint a 
bejelentkezés, vicc feltöltése, kedvelés, viccek rendzserezése.
A felhasználók által feltöltött vicceket lehet értékelni, egy egyszerű 
tetszik-nem tetszik rendszer segítségével.

## A jelenlegi helyzet leírása

Sipos Valentin Dominik mint magánszemély kérvényézte egy viccportál készítését 
a Kovácspál Bálint István Kft-től.
Sok viccet hall az egyetemen, azonban nem tudja mindet fejben tartani, illetve 
megosztani másokkal, ezért szeretne egy modern, letisztult, megosztófelületet 
a vicceinek, ahol lehetőség nylik más emberek számára is megosztaniuk az életük
humoros részeit.
Az interneten jelenlévő többi viccportállal ellentétben, ez egy alkalmazás 
lesz, mely különösen az EKKE C épületnél történő dolgokat célozza meg.

## Vágyalomrendszer

Olyan asztali alkalmazás létrehozása, amely minden PC-ről tökéletesen elérhető 
és bármely képernyő mérethez igazítható.
Ennek megoldásához tökéletes egy Electron alkalmazás létrehozása, egy adatbázis
segítségével.
A képernyő méretbeli különbségeket reszponzív megoldással tudnánk megoldani.
Fontos lenne, hogy ne csak egy operációs rendszeren működjön, hanem Windowson, 
Linuxon és MacOs-en.
Törekedünk olyan kódbázis kialakítására, melynek a bővítése a későbbiekben nem 
okoz, majd gondot.
Törekvés modern technológiák, keretrendszerek, stílusok és konvenciók 
használatára, mint a Rust és az Electron.
Fontos a felhasználók szétválasztása különböző jogosultságok szerint. 
Ezek az admin, akik tudnak posztot törölni.
Felhasználók, akik vicceket írnak és pontozzák a többi viccet.
Látogatók, akik csak olvasni tudják a poénokat.

## A jelenlegi üzleti folyamatok modellje

+ Igényfelmérés: Felhasználók formon keresztüli igényfelmérése, hogy a jövőben 
milyen funkciókat használnának.
+ Regisztráció, adatok kezelése: A látogatók képesek egy regisztrációs form 
kitöltése után saját fiókot nyitni, azokban az adataikat 
kezelni / megváltoztatni.
+ Adminisztrátorok keresése: Probléma és hibajelentés esetén, az adminok 
eléréseit lehet megtekinteni

## Igényelt üzleti folyamatok modellje

+ Posztok jutalmazása: A felhasználók pontokkal illethetik a számukra 
szimpatikus posztokat.
+ Szűrés a legkedveltebb posztok között: A posztokat az előbb említett 
pontrendszer alapján szűrni is lehessen, pl.: A legkedveltebb legelsőnek 
jelenik meg.
+ Saját profil szerkesztés: A regisztrált felhasználók a saját fiókjukat 
tudják szerkeszteni, pl.: név szerkesztés.
+ Rang rendszer: A látogatók három rangba lesznek besorolva (Fejlesztő, 
Felhasználó, Látogató), amik más-más hozzáféréssel rendelkeznek az oldal 
tartalmaihoz.
## Követelménylista

| Id    | Modul     | Név           | Leírás |
| :---: | ---       | ---           | :---   |
| K1    | Felület   | Bejelentkezés | A felhasználók itt tudnak bejelentkezni a rendszerbe, ha bejegyzést szeretnének létrehozni. Probléma esetén üzenetet küldeni a rendszert          karbantartóknak, illetve a saját profiljával felmerült problémákat javíthatja, ez lehet elfelejtett vagy jelszó. |
| K2    | Felület   | Bejegyzések   | A felhasználók az oldal megnyitása után kezdőképernyőként a Bejegyzések nevű oldalra kerülnek, ahol megtekinthetik az aktuális posztokat, bejelentkezés után interaktálhatnak azokkal, bejelentkezés nélkül csak a megtekintésükre van mód. |
| K3    | Felület   | Kategóriák    | A felhasználók választani tudnak a megtekinteni kívánt tartalmak között, itt megjelennek formátum és tartalom szerint szétbontva. |
| K4    | Felület   | Profil        | A felhasználók a saját profiljuk módosításait eszközölhetik, láthatóvá válnak az adott bejegyzésekre küldött interakciók, pontozások is.|
| K5    | Felület   | Bejegyzés létrehozása | A felhasználó beküldhet saját viccet, mém-et vagy gif-et, amelyet megtekinthet a főoldali bejegyzések között. |
| K6    | Felület   | Regisztráció  | A felhasználók ezen az oldalon tudnak saját fiókot létrehozni a rendszerben, így jogokat kapni a saját tartalmaik megosztására.|
| K7   | Modifikáció | Elfelejtett jelszó | Ha a felhasználó nem emlékszik a megadott jelszavára, akkor itt van lehetősége egy adminhoz fordulni, vagy újat generálni. |
| K8   | Modifikáció | Jelszó módosítása  | A felhasználónak lehetősége van módosítani saját jelszavát, megadva a régi majd az új verziót. |
| K9   | Modifikáció | Felhasználó módosítása | A felhasználónak lehetősége van saját felhasználónevének módosítására |
| K10   | Jogosultság | Jogosultsági szintek | - Admin: bejegyzések törlése, - Felhasználó: tartalmak megtekintése, saját bejegyzések létrehozása.<br> - Látogató: bejegyzések megtekintése. |
## Fogalomtár

+ **POSZT** __(angol: post)__  A felhasználók által a közösségi felületekre 
beküldött / feltöltött szöveges vagy audiovizuális tartalom. 
+ **ADMIN** __(angol: admin)__ Az oldal adminisztrálásáért felelős személy
+ **MÉM** __(angol: meme)__ Egy humoros kép, vagy szöveg, amit gyorsan 
másolnak és osztanak meg az interneten