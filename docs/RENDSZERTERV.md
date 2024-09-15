# Rendszerterv

## A rendszer célja
Az oldal célja az, hogy az informatikai eszközök kihasználásával létrejöjjön egy
olyan oldal, amely mosolyt csal a szürke hétköznapokban az azt látogatóinak.
Annak érdekében, hogy ez a rendszer fenntartható legyen és ne válljon egy olyan
oldallá, amely már nem az eredeti célokra szolgál rákellett jönnünk, hogy ellenőrzött
módon kell, hogy működjön. Ennek érdekében az weblap különböző feladatkörrel
rendelkező jogosultságokkal rendelkezik. A regisztráláskor a rendszer automatikusan
a felhasználók jogosultságait rendeli hozzá a profilokhoz, mellyel az ezzel rendelkező
felhasználók könnyen tudnak egy újabb posztot ellenőrzésre küldeni, vagy egy, már
az oldalon szereplő bejegyzést véleményezni egy arra kifejlesztett pontozórendszerrel. 
Azok akik nem regisztrálnak fel az oldalra, azok "olvasói jogot" kapnak, mely kimerül
abban, hogy látják a posztokat, viszont nem tudnak egy új bejegyzést létrehozni, és a
korábbiakat pontozni sem. Ezen felül létezik még a szerkesztői jog, mellyel látni
tudjuk az éppen arra várakozó posztok listáját, mellyek az ellenőrzésre várnak,
illetve azok kiengedését az oldalra tudjuk még engedni. Ezt a jogosultságot csak is
kizárólag az adminnak van joga bárkinek is adni. Az admin rendelkezik az összes olyan
funkcióval, mely elérhető az összes jogosultságban, azontúl jogokat tud hozzárendelni
az egyes felhasználókhoz, vagy elvenni azokat. Mivel az oldalt leginkább a látogatók
és az alkotók fogják használni a legtöbbször, ezért célunk, hogy a rendszer a lehető
legközérthetőbb legyen és beszédes. Fontos még, hogy az oldal ilyen széleskörű
használata miatt a design-nak mindenféleképp, hogy responsive-nak kell lennie.
Ezzel azt tudjuk megelőzni, hogy ne történhessen meg az, hogy az élvezeten rontson
az összecsúszott oldal. A responsive oldalak nagy előnye, hogy szinte képes majdnem
minden képernyőmérethez igazodni, ezért nekünk is ezt a módszert kell alkalmaznunk.
Nem szabad megengedni azt sem, hogy valaki elveszve érezze magát az oldalon, ezért
ha szükséges, mindenhez is teszünk egy súgó felületet, mellyel máris könnyebb lesz a kezelése. 
Ezen felül viszont a rendszer nem célja az, hogy az oldal által is kiválasztott és
használt témán felül bármi másra is alkalmas legyen, vagy hogy egyszere több cég is
használni tudja a későbbiekben, hisz egyenlőre csak egyetlen egy kiszolgálára van
csak tervezve.

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

### Funkcionális követelmények

+ Bejegyzések tárolása (képes képet vagy más formátumot pl. gif-et kezelni)
+ Felhasználók adatainak tárolása (felhasználónév, jelszó, email)
+ Webes környezeten működőképes (reszponzív, így számítógépen / telefonon is megjeleníthető)
+ Képes úgy megjelíteni posztokat, hogy nincs aktív bejelentkezett felhasználó
+ Bejelentkezett felhasználó reakciót adhat különböző más felhasználók tevékenységeire
+ A webes felület gördülékenyen kommunikál a hozzárendelt adatbázissal

### Nemfunkcionális követelmények

- Felhasználók nem módosíthatnak más felhasználók bejegyzéseit
- Felhasználók nem férhetnek hozzá más felhasználók személyes adataihoz csak a felhasználónevüket és profilképüket láthatják
- Az oldal könnyen kezelhető, minimális informatikai tudással igénybe vehető
- Webes felületen kezel adatbázist
- Az oldal eléréséhez egy okoseszközre és internethozzáférésre van szükség, így nem igényel különleges hardveres támogatást
- Használatához telepítés nem szükséges, különböző kiegészítők használata nélkül is elérhető

## Funkcionális terv

### 5.1 Rendszerszereplők

+ **ADMIN**
    - a rendszer felügyelete az elsődleges feladata, ennek tesztelésére minden szerepkörbe be tud lépni, hogy ellenőrizze azok hibamentes üzemelését

+ **FELHASZNÁLÓ**
    - jogában áll az oldalon megjelenő minden tartalom megtekintése, valamint az ezekre adott visszajelző rendszerek használata
    - különböző bejegyzéseket posztolhat, más bejegyzésekre pontokat adhat

+ **LÁTOGATÓ**
    - az oldalon megjelenő bejegyzéseket tekintheti meg
    - a bejegyzéssel nem interaktálhat csak ha rendelkezik profillal

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
