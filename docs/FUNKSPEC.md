# Funkcionális specifikáció

## Jelenlegi helyzet leírása
Sipos Valentin Dominik magánszemélyként kérvényezte egy viccportál elkészítését a Kovácspál Bálint István Kft-től. Egyetemi évei alatt rengeteg viccet hall, azonban nem mindig képes mindet megjegyezni vagy megosztani másokkal. Emiatt szeretne létrehozni egy modern, letisztult platformot, amely lehetőséget biztosít viccei megosztására, valamint más emberek számára is lehetővé teszi, hogy életük humoros pillanatait megosszák.
Ez a portál egyedi lenne az interneten már meglévő viccoldalakkal szemben, mivel nem egy hagyományos weboldal, hanem egy applikáció formájában valósulna meg. Az alkalmazás különlegessége, hogy kifejezetten az EKKE C épületnél történő humoros eseményeket célozza meg, így egy specifikus közösség számára kínál majd lehetőséget a szórakozásra és az élmények megosztására.
## Vágyalomrendszer leírása
Olyan asztali alkalmazás létrehozása, amely minden PC-ről tökéletesen elérhető
és bármely képernyőmérethez igazítható.\
Fontos lenne, hogy az asztali alkalmazás bármilyen operációs rendszeren elérhető
legyen, hiszen nem csak egy platformra szeretnénk elérhetővé tenni a portált.\
Ennek megoldásához egy Electron keretrendszerben írt alkalmazás létrehozása 
tökéletes, szerverként pedig egy REST API megoldás a megfelelő, amely kommunikál
egy adatbázissal.\
A képernyő méretbeli különbségeit reszponzív megoldással lehetne megoldani, ez
CSS-ben megoldható.\
A frontend dinamikusságára mindenképpen JavaScript használata szükséges, viszont
a statikus tipizáláshoz a Microsoft által készített TypeScript használata még 
jobb megoldás.\
A szervert mindenképpen Rust nyelven szeretnénk megvalósítani, hiszen gyorsasága
és biztonsága fontos a megfelelő működéshez akár a jövőben is. Az adatbázis
kezelésére SQL nyelv az optimális, hiszen ez pontosan erre lett kitalálva.
Célunk, hogy tiszta kódot kapjunk, hogy a későbbiekben csatlakozó webfejlesztő
munkatársak bármikor könnyedén tudjanak csatlakozni a fejlesztéshez.\
Fontos lenne, ha a portálon megkülönböztetnénk a felhasználókat és adminisztrátorokat, 
hisz nem szeretnénk, hogy bármilyen rossz indulatú felhasználó be tudjon nyúlni 
a portálba, akár törölni tartalmat.\ 
Erre tökéletes megoldás a rangok besorolása, számunkra 3 főbb rangra lenne szükség:
Admin, Felhasználó és Látogató, természetesen nem lenne baj, ha ezt később tudnánk
bővíteni, akár egy Moderátor státusszal.\
Az adatbázis séma szempontjából nincsenek nagy elvárások, viszont mindenképpen
fontos, hogy a különböző egyedeket külön táblában tároljuk, és skálázás későbbiekben
megoldható legyen.\
A portál tényleges kinézetére való terv a Képernyőtervek pont alatt található
a terv.\
Ezek ötvözése úgy gondoljuk, hogy egy egyszerűen továbbfejleszthető, skálázható
portál készítésére alkalmas, amely megfelel minden követelmények.

## Jelenlegi üzleti folyamatok modellje
A mai világban az internetes viccoldalak gyakran elavultnak tűnnek, mivel már nem képesek lépést tartani a modern humorral és a társadalmi változásokkal. Ezek a portálok tele vannak régi, évekkel ezelőtt terjesztett poénokkal, amelyek mára elvesztették frissességüket és aktualitásukat.
A humorérzék az idővel változik, és a régebbi viccoldalak sok esetben már nem tudnak vonzóak lenni a mai közönség számára. Bár néhányan nosztalgiából még élvezik ezeket a régi vicceket, az újdonságot keresők gyakran hosszú időt töltenek az interneten a friss és humoros tartalmak felkutatásával.
Ez a folyamat időigényes, főleg akkor, ha valaki a saját ízlésének megfelelő vicceket szeretne találni. Az elavult viccoldalak nem képesek az új humorstílusokat és a változó közönségigényeket kielégíteni, így egyre inkább szükség van olyan platformokra, amelyek releváns, friss poénokkal szórakoztatják a felhasználókat.
Az egyetem C épületénél létrejött humoros viccek, azoknak szól leginkább, akik sok időt töltenek körülötte.
## Igényelt üzleti folyamatok modellje
A platformon a felhasználók értékelhetik a posztokat pontokkal, így könnyedén jutalmazhatják a számukra kedves tartalmakat. Az értékelési rendszer lehetővé teszi, hogy a legkedveltebb posztok az oldal tetején jelenjenek meg, így a látogatók könnyen megtalálhatják a közönség által leginkább kedvelt bejegyzéseket. A regisztrált felhasználók testreszabhatják saját profiljukat, például módosíthatják a nevüket. A rendszer három különböző rangba sorolja a látogatókat – Fejlesztő, Felhasználó és Látogató –, és ezek a rangok különböző hozzáférési szinteket biztosítanak az oldal tartalmaihoz.
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
## Használati esetek
+ **ADMIN**: Feladata a rendszer felügyelete, karbantartása, tesztelése. Ebből 
következően minden szerepkörbe be tud lépni, hogy ellenőrizze azok hibamentes 
üzemelését. Joguk van egy bejegyzést, kommentet törölni, vagy módosítani,
hiszen feladatuk moderálni a portálon való tartalmakat.
+ **FELHASZNÁLÓ**: Jogában áll az oldalon megjelenő minden tartalom megtekintése,
valamit ezekre adhat visszajelzéseket upvote-downvote formátumban. Bejegyzéseket
posztolhat, mások bejegyzésére írhat megjegyzéseket, illetve azokat is értékelheti.
+ **MEGTEKINTŐ**: Az oldalon megjelenő bejegyzéseket, illetve ahhoz hozzászólásokat
megtekintheti, viszont ezeket nem értékelheti, illetve nem írhat új hozzászólást.

## Megfeleltetés, hogyan fedik le a használati eseteket a követelmények
A megadott modulok lefedik a legtöbb kulcsfontosságú funkciót, amelyek lehetővé teszik a felhasználók számára a viccportál hatékony használatát. Az alábbiakban áttekintem, hogyan fedik le a követelmények a használati eseteket:

+ **K1** Bejelentkezés: A felhasználók bejelentkezhetnek a rendszerbe, ami előfeltétele a posztok létrehozásának és interakcióknak. Továbbá problémamegoldási lehetőségeket biztosít, mint például a jelszó-helyreállítás.

+ **K2** Bejegyzések: A kezdőképernyőként szolgáló modul biztosítja a felhasználók számára az aktuális posztok megtekintését és az interakciót a bejegyzésekkel (ha bejelentkeztek). Ez az oldal kulcsfontosságú a viccportál alapvető használati esetéhez, hiszen itt történik a viccek, mémek és egyéb tartalmak fogyasztása és értékelése.

+ **K3** Kategóriák: A felhasználók a tartalmakat különböző kategóriák alapján szűrhetik, ami lehetővé teszi számukra a preferált viccek, mémek vagy gif-ek könnyű megtalálását. Ez a funkció releváns azok számára, akik specifikus típusú tartalmakat keresnek.

+ **K4** Profil: A saját profil szerkesztése és a korábbi interakciók, pontozások megtekintése biztosítja, hogy a felhasználók nyomon követhessék tevékenységüket, és személyre szabhassák fiókjukat. Ez fontos a felhasználói élmény fokozásában.

+ **K5** Bejegyzés létrehozása: A felhasználók saját vicceket, mémeket vagy gif-eket küldhetnek be, ami a felhasználói generált tartalmak szempontjából alapvető funkció. Ez a funkció közvetlenül kapcsolódik a felhasználói tartalom létrehozásához és az oldal aktív használatához.

+ **K6** Regisztráció: A regisztráció lehetősége biztosítja, hogy a felhasználók saját fiókot hozzanak létre, így teljes hozzáférést kapjanak a tartalom létrehozásához és interakcióhoz.

+ **K7** Elfelejtett jelszó: Jelszó-helyreállítási lehetőség biztosítja, hogy a felhasználók akkor is hozzáférjenek a fiókjukhoz, ha elfelejtik jelszavukat. Ez támogatja a gördülékeny felhasználói élményt.

+ **K8** Jelszó módosítása: A felhasználók bármikor megváltoztathatják jelszavukat, növelve ezzel a fiókbiztonságot és a kontrollt.

+ **K9** Felhasználó módosítása: A felhasználónév módosításának lehetősége lehetővé teszi a profil további testreszabását.

+ **K10** Jogosultsági szintek: A jogosultsági szintek (Admin, Felhasználó, Látogató) megfelelően szabályozzák a hozzáférést a portál különböző funkcióihoz. Az adminisztrátorok törölhetnek bejegyzéseket, míg a felhasználók új tartalmat hozhatnak létre, a látogatók pedig csak megtekinthetik az oldal tartalmait. Ez a hierarchia biztosítja, hogy mindenki az adott szerepkörének megfelelően használhassa a rendszert.

## Képernyőtervek
| kép | Leírás|
| :--:|:-----:|
| ![fooldal.png](https://github.com/Valesz1022/AFP1_kisProjekt/blob/main/images/fooldal.png) | A  főoldal terve |
| ![fooldal_admin.png](https://github.com/Valesz1022/AFP1_kisProjekt/blob/main/images/fooldal_admin.png) | A főoldal kinézete admin szemszögből |
| ![fooldal_felhasznalo.png](https://github.com/Valesz1022/AFP1_kisProjekt/blob/main/images/fooldal_felhasznalo.png) | A főoldal kinézete egy mezei felhasználó szemszögéből |
| ![Bejelentkezes.png](https://github.com/Valesz1022/AFP1_kisProjekt/blob/main/images/Bejelentkezes.png) | A bejelentkezés oldal terve |
| ![Regisztracio.png](https://github.com/Valesz1022/AFP1_kisProjekt/blob/main/images/Regisztracio.png) | A regisztrációs oldal terve |
| ![Uj_poszt.png](https://github.com/Valesz1022/AFP1_kisProjekt/blob/main/images/Uj_poszt.png) | Az új vicc posztolásához felugró ablak


## Forgatókönyvek
### 9.1 Alap forgatókönyv
Amikor a felhasználó megnyitja a weblapot, az Kezdőlap jelenik meg számára, ahol megtalálható az oldal címe, valamint a navigációs sáv, amelyből a **Bejelentkezés** és **Regisztráció** opciók érhetők el. A baloldalon található műfajokból lehet válogatni, melyek segítségével a látogató az adott műfajhoz tartozó tartalmakat böngészheti. Alapértelmezettként a legjobban értékelt bejegyzések toplistáját látja, a legmagasabb pontszámú tartalomtól a legkevésbé értékeltig.

A Regisztráció gombra kattintva egy regisztrációs panel jelenik meg, ahol a látogatók néhány adat megadásával regisztrálhatnak felhasználóként.
A Belépés opció kiválasztása után a felhasználók a felhasználónév és jelszó megadásával jelentkezhetnek be az oldalra. Hibás adat esetén a rendszer újra kéri a bejelentkezési adatokat.
Amennyiben **Felhasználói** jogkörrel rendelkező felhasználó jelentkezik be, a kezdőoldal bővített információkat nyújt. A felhasználó láthatja az általa adott pontszámokat és az összpontszámokat az egyes bejegyzésekhez. Megjelenik egy gomb is, amelyen egy **+** ikon látható; erre kattintva érhető el a Bejegyzés létrehozása menüpont, ahol új tartalmat lehet beküldeni. Az új bejegyzéseket egy moderátor ellenőrzi, és engedélyezés után jelennek meg az oldalon. A belépést követően a **Regisztráció** gomb **Profil szerkesztése** gombbá alakul át, ahol a felhasználó szerkesztheti a profilját és módosíthatja a jelszavát.

Ha valaki **Moderátori** jogkörrel jelentkezik be, további lehetőségek is megjelennek, például a Bejegyzések engedélyezése menüpont, ahol a felhasználók által beküldött bejegyzéseket lehet megtekinteni, és dönteni azok megjelenítéséről vagy elutasításáról.

Amennyiben a felhasználó **Adminisztrátori** jogkörrel jelentkezik be, minden korábbi jogkörrel rendelkező funkció elérhető lesz számára, és hozzáférést kap egy Jogosultságok menüponthoz is, ahol módosíthatja a regisztrált felhasználók jogkörét vagy akár törölheti is a felhasználókat az oldalról.

### 9.2 Jogosultsági szintek lehetőségei
__Megtekintő__ (nem bejelentkezett felhasználóként) a következő lehetőségek állnak rendelkezésre:

**Bejelentkezés**: A felhasználók, akik már rendelkeznek fiókkal, itt adhatják meg bejelentkezési adataikat.
**Regisztráció**: Az új felhasználók itt regisztrálhatnak fiókot.
Alapértelmezett toplistát láthatnak, ahol a legjobban értékelt tartalmak jelennek meg először.
Kiválaszthatnak egy kategóriát, például az Apa viccek kategóriát, és megnézhetik a legjobb bejegyzéseket ebben a kategóriában.
Láthatják az értékelések átlagát minden bejegyzésnél.
**Felhasználói** jogkörrel rendelkező felhasználók a következőket tehetik:

Minden fent említett műveletet elvégezhetnek.
__Kijelentkezés__: Kiléphetnek a fiókjukból, és visszakerülhetnek a megtekintő módba.
__Profil módosítása__: A felhasználók módosíthatják a profiljukat, beleértve a profilképüket, megjelenített nevüket és jelszavukat. Jelszó módosításához meg kell adniuk a régi jelszót is.
__Bejegyzés létrehozása__: Új bejegyzést küldhetnek be egy űrlap kitöltésével, ahol meg kell adniuk a bejegyzés címét és leírását, valamint képet is csatolhatnak. Az **elküldött** bejegyzéseket moderátorok ellenőrzik.
A felhasználók értékelhetik a bejegyzéseket egy 1-10-es skálán.
Megtekinthetik az általuk adott pontszámot és az átlagos pontszámot minden bejegyzésnél, beleértve a sajátjaikat is.
Bejelenthetnek más felhasználók által beküldött bejegyzéseket felülvizsgálatra.
**Moderátori** jogkörrel rendelkezők további lehetőségei:

Minden korábban felsorolt funkciót elérhetnek.
Az **Engedélyezés** menüpontban megtekinthetik és engedélyezhetik vagy elutasíthatják a felhasználók által beküldött bejegyzéseket.
**Adminisztrátori** jogkörrel rendelkező felhasználók lehetőségei:

Minden korábban felsorolt funkciót elérhetnek.
A __Jogosultságok__ menüpontban módosíthatják a regisztrált felhasználók jogköreit, vagy eltávolíthatják őket az oldalról, például sértő viselkedés esetén.
## Funkció - követelmény megfeleltetése
|Id|	|Követelmény|	                                    |Funkció|
K1	Bejelentkezés nélküli interakciók elkerülése	A főoldalon a pontozási funkció alapértelmezetten le van tiltva, és csak bejelentkezett felhasználók számára elérhető.
K2	Tartalmak kategorizálása	                    A navigációs sávon egy "Kategóriák" legördülő lista segítségével választhatják ki a felhasználók, hogy milyen típusú tartalmakat szeretnének megtekinteni.
K3	Profil szerkesztése	                            A felhasználók a "Profil" gombra kattintva szerkeszthetik azokat az adatokat, amelyeket a regisztráció során adtak meg, valamint megtekinthetik a kedvelt tartalmakat.
K4	Saját tartalom feltöltése	                    A felhasználók a főoldalon tölthetnek fel saját tartalmat, amely lehet szöveges bejegyzés, GIF vagy kép. A feltöltött tartalom csak admin vagy moderátor jóváhagyása után jelenik meg.
K5	Toplista megjelenítése	                        A főoldalon a posztok egy toplistában jelennek meg, amit a felhasználók személyre szabott szűrőkkel módosíthatnak.
K6	Admin felület	                                Admin jogosultságú felhasználóknak egy külön gomb jelenik meg a navigációs sávban, amelyre kattintva megtekinthetik a felhasználói tevékenységeket, és szükség esetén felfüggeszthetik a fiókokat.
K7	Moderátori felület	                            A moderátoroknak is külön felület érhető el a navigációs sávban, ahol megtekinthetik a függőben lévő posztokat, és eldönthetik azok jóváhagyását vagy törlését. Moderátorok adminoknak is jelenthetik a felhasználókat.
K8, K9	Bejelentkezési és regisztrációs felület	    A bejelentkezési oldalon lehetőség van a bejelentkezésre vagy regisztrációra. Elfelejtett felhasználónév vagy jelszó esetén a felhasználók moderátorhoz fordulhatnak, illetve új fiókot hozhatnak létre. A bejelentkezés/regisztráció után a felhasználók szabadon reagálhatnak a tartalmakra és tölthetnek fel saját bejegyzést.
K10	Elfelejtett felhasználónév vagy jelszó	        A bejelentkezési oldalon a felhasználók kérhetnek új jelszót, ha elfelejtették a jelenlegit.
K11	Jelszó módosítása	                            A belépést követően a profilbeállítások között a felhasználók megadhatják a régi és az új jelszavukat a jelszó módosításához.
K12	Becenév módosítása	                            A posztoláshoz és reagáláshoz a felhasználóknak regisztrációkor meg kell adniuk egy becenevet, amelyet a profil oldalon bármikor módosíthatnak. A rendszer ellenőrzi, hogy a becenév foglalt-e.
K13	Jogosultsági szintek	                        A regisztráció után a felhasználók automatikusan felhasználói jogosultságot kapnak. Magasabb jogosultságot (pl. admin vagy moderátor) csak adminisztrátori jogokkal rendelkező felhasználók oszthatnak ki.
## Fogalomszótár
+ **ADMIN** _(adminisztrátor)_ Oldal kezelője, lehetősége van tartalom törlésére.
+ **USER** _(felhasználó)_ Oldal fiókkal rendelkező felhasználója.
+ **GUEST** _(vendég)_ Oldal fiókkal nem rendelkező látogatója.
+ **AUTHOR** _(szerző)_ Adott vicc szerzője.
+ **CONTENT** _(tartalom)_ Adott vicc tartalma.
+ **UPVOTE** _(felszavazás)_ Tetszés kifejezésére ad lehetőséget a viccek alatt.
+ **DOWNVOTE** _(leszavazás)_ Nem tetszés kifejezésére ad lehetőséget a viccek alatt.
+ **SAVED** _(mentett)_ Felhasználó által elmentett vicc, amelyet később visszanézhet.
