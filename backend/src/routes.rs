//! Minden végpont ebbe a modulba kerül, almodulokra felbontva külön-külön
//! URL alapján. Egy-egy adott végponton belül az adott HTTP method nevével
//! található függvény a tényleges kontroller.

pub mod health_check;
