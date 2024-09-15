# Funkcionális specifikáció

## Jelenlegi helyzet leírása

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

## Igényelt üzleti folyamatok modellje

## Követelménylista

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

## Képernyőtervek

## Forgatókönyvek

## Funkció - követelmény megfeleltetése

## Fogalomszótár
+ **ADMIN** _(adminisztrátor)_ Oldal kezelője, lehetősége van tartalom törlésére.
+ **USER** _(felhasználó)_ Oldal fiókkal rendelkező felhasználója.
+ **GUEST** _(vendég)_ Oldal fiókkal nem rendelkező látogatója.
+ **AUTHOR** _(szerző)_ Adott vicc szerzője.
+ **CONTENT** _(tartalom)_ Adott vicc tartalma.
+ **UPVOTE** _(felszavazás)_ Tetszés kifejezésére ad lehetőséget a viccek alatt.
+ **DOWNVOTE** _(leszavazás)_ Nem tetszés kifejezésére ad lehetőséget a viccek alatt.
+ **SAVED** _(mentett)_ Felhasználó által elmentett vicc, amelyet később visszanézhet.
