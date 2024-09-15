# Rendszerterv

## A rendszer célja

## Projektterv

### Projektmunkások és felelőségek:
* Frontend:
    + Pászti Szabolcs
    + Gunics Roland
* Feladatuk:
    + A felhasználók által elérhető felületek létrehozása. 
    + A backend által biztosított funkciók igénybevétele.
    + Egységes design megalkotása.
* Backend:
    + Zana Domán
    + Molnár Máté Norbert
* Feladatuk: 
    + A frontend kiszolgálása adatokkal.
    + Adatbázisok létrehozása.
* Tesztelés:
    + Zana Domán
    + Molnár Máté Norbert
    + Pászti Szabolcs
    + Gunics Roland
* Feladatuk: Hibák feltárása az üzembe helyezés előtt.

## Üzleti folyamatok modellje

## Követelmények

## Funkcionális terv

## Fizikai környezet
- Az alkalmazás asztali platformra készült, amely elérhető bármilyen PC-ről,
internet hozzáférés mellett.
- Semmilyen egyéb alkalmazás telepítése nem szükséges az alkalmazás futtatásához.

## Architekturális terv
### Backend szerver
- A webszerver tokio futtatási környezet alatt fut, amely egy több szálon futó, 
aszinkron futtatási idő Rust nyelvhez.
- Axum keretrendszerre épül, ez adja az útválasztót, illetve végpontok írásához
szükséges eszközöket.
- Rust programozási nyelven íródott, amely egy rendszerszintű programozási
nyelv, célja, hogy biztonságos és gyors is legyen.
- REST architektúrán íródott, és JSON formátumban kéri/várja a HTTP 
kéréseket/válaszokat.
### Frontend kliens
- Electron keretrendszerrel íródott, amely egy olyan nyílt forráskódú 
keretrendszer, amely segítségével webes technológiákat, pl HTML, CSS, JavaScript,
lehet használni asztali alkalmazások létrehozására.
- TypeScript programozási nyelven íródott, amely egy JavaScript-alapú programozási
nyelv, lehetővé teszi statikus típusok használatát.

## Adatbázis terv

## Implementációs terv
### Backend
A webszerveren futú Rust alkalmazás és adatbázis tartalmazza az üzleti logikát,
ezzel végpontokon keresztül lehet HTTP protokollnak megfelelően REST 
architektúrának megfelelő kérésekkel komunnikálni, és állapotát módosítani.\
MariaDB relációs adatbázis-kezelő rendszer fut a szerveren az adatok tárolására,
kezelésére és karbantartására.
### Frontend
A felhasználói felület, amely asztali alkalmazásban megjelenő weboldal 
formájában érhető el, HTML, CSS, TypeScript nyelven íródott, Electron 
keretrendszer segítségével asztali alkalmazásként jelennek meg ezek a weblapok.

## Tesztterv

## Telepítési terv
+ **Fizikai telepítési terv**: Egy szervergépre MariaDB szoftver telepítése és
beüzemelése szükséges, majd erre szükséges feltölteni az adatbázis sémát a
megfelelő fájlokkal. Ugyanezen a szerveren szükséges a cargo parancssori
alkalmazás telepítése, és ezen keresztül való építése, futtatása a Rust 
forráskódnak.
+ **Szoftver telepítési terv**: A portál eléréséhez elég az alkalmazás
letöltése, telepítése, és futtatása egy olyan PC-n, amely internethez csatlakozik.

## Karbantartási terv
### Tartalmi karbantartás
A portálon megjelenő feltöltött tartalmak folyamatos ellenőrzés alatt állnak,
a nem kért, káros tartalmak törlésre kerülnek.\
Időszakosan végezhető karbantartás, akár automatizálva is a regisztrált
felhasználók neveinek ellenőrzése az esetlegesen előforduló sértő, káros szavak
használata ellen.\
Szintén időszakosan végezhető karbantartás, a troll felhasználók elleni
védelem, akik gyakran sértő megjegyzéseket tesznek más felhasználók tartalma
alá.
### Rendszeres karbantartás
+ **Adatbázis karbantartása**: Az adatbázisséma úgy van felépítve, hogy
automatikusan tisztul a törölt felhasználók posztjaitól, illetve a portálra
feltöltött tartalmak régi verziójától.\
Időszakosan végzett karbantartás a felesleges adatok törlése, pl. spam kommentek,
vagy inaktív felhasználók.\
Az adatbázisról rendszeresen biztonsági mentés készül a szerverre.
+ **Szerver karbantartása**: A szerver forráskódja megírása közben arra
törekedünk, hogy későbbiekben a kódhoz ne kelljen hozzányúlni karbantartási
célból (pl. függőségeknek elmentjük verziószámát).
