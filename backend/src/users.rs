//! Modul a felhasználók, hitelesítés és jogosultságok kezelésére.
//!
//! `axum-login` crate-t felhasználva a backend adatbázisán menti el a
//! jelenleg bejelentkezett felhasználókat, illetve a hozzájuk tartozó sütiket.

use core::{
    fmt::{self, Debug, Formatter},
    hash::Hash,
};
use std::collections::HashSet;

use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use thiserror::Error;
use tokio::task;

/// A felhasználók modellje, megegyezően az adatbázisban tárolt adatokkal.
#[derive(Clone, Serialize, FromRow)]
pub struct User {
    name: String,
    password: String,
    admin: bool,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("password", &"******") // nem szeretnénk véletlenül leakelni
            .field("admin", &self.admin)
            .finish()
    }
}

/// Az `axum-login` crate elvárja, hogy a felhasználókhoz implementáljuk az
/// `AuthUser` trait-et, ezzel fel tudjuk használni a könyvtár minden lehetőségét.
impl AuthUser for User {
    type Id = String;

    // Mivel a felhasználóknak az adatbázisban nincsen egyéni azonosítójuk,
    // a nevet használjuk fel erre.
    fn id(&self) -> Self::Id {
        self.name.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

/// A hitelesítő adatok, amelyeket bekérünk a frontendről. Ezeket ellenőrzi
/// az adatbázissal, hogy létezik-e ilyen felhasználó.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub name: String,
    pub password: String,
}

/// Az `axum-login` crate elvárja, hogy készítsünk egy Backend struktúrát az
/// adatbázishoz, amely a hitelesítés és jogosultságok kezelésére szolgál,
/// tartalmazza az adatbázissal való kapcsolatot.
#[derive(Debug, Clone)]
pub struct Backend {
    db: MySqlPool,
}

impl Backend {
    pub const fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}

/// A hitelesítés során felmerülhető problémák.
#[derive(Debug, Error)]
pub enum AuthError {
    /// Az adatbázissal valami hiba történt.
    #[error(transparent)]
    MySql(#[from] sqlx::Error),
    /// A háttérben futó hitelesítő szálon történt valami hiba.
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

/// A hitelesítő trait implementálása az `axum-login` elvárásai szerint.
///
/// Hitelesítés, és ellenőrzésen kívül minden mást a könyvtár végez. Ha sikeres
/// a hitelesítés, elmenti az adatbázisba a felhasználót jelenleg
/// bejelentkezettként.
#[async_trait]
impl AuthnBackend for Backend {
    type Credentials = Credentials;
    type Error = AuthError;
    type User = User;

    /// Hitelesít egy felhasználót, ellenőrzi, hogy létezik-e ez a felhasználó,
    /// illetve a jelszava megegyezik-e az elmentettel. Ehhez argon2
    /// titkosítási módszert használunk, a megkapott, illetve elmentett jelszó
    /// titkosítótt változatát hasonlítja össze.
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> =
            sqlx::query_as("SELECT * FROM users WHERE name = ?")
                .bind(creds.name)
                .fetch_optional(&self.db)
                .await?;

        task::spawn_blocking(|| {
            Ok(user.filter(|user| {
                password_auth::verify_password(creds.password, &user.password)
                    .is_ok()
            }))
        })
        .await?
    }

    /// Ellenőrzi, hogy létezik-e olyan felhasználó, amelyet a megadott id-val
    /// kap.
    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("SELECT * FROM users WHERE name = ?")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}

/// A jogosultságokra az `axum-login` könyvtár egy olyan generikus eszközt ad,
/// amellyel lehet akár több jogosultságot megadni egy felhasználónak.
/// Nekünk ez most nem kell, viszont ha később kellene, akkor is jobb már előre
/// a könyvtár elvárásai szerint dolgozni, nem pedig kerülöutakon, hiszen nem
/// annyira teljesítményigényes, hogy feltűnő legyen.
#[derive(Debug, Clone, Eq, PartialEq, Hash, FromRow)]
pub struct Permission {
    pub name: String,
}

/// Segítő trait a szövegből Permission-be való konvertáláshoz.
impl From<&str> for Permission {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_owned(),
        }
    }
}

/// A jogosultságokat kezelő trait implementálása az `axum-login` elvárásai
/// szerint.
///
/// Attól függően, hogy milyen jogosultsága van egy adott felhasználónak, a
/// különböző végpontokat jogosultságok mögé tehetjük, és nem érheti el azokat
/// addig, amíg nem jelentkezik be egy olyan felhasználóval, akinek már van
/// jogosultsága hozzá.
#[async_trait]
impl AuthzBackend for Backend {
    type Permission = Permission;

    /// Csak egy függvény implementálását várja, amellyel megadhatjuk, hogy egy
    /// adott felhasználó csoportjában milyen jogosultságok vannak.
    /// Ez itt nálunk kettő lehetőség lehet: admin, vagy nem admin, ezért
    /// az adatbázisnak már nem is kell lekérdezést küldeni, hiszen ez már
    /// el van mentve.
    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        // Ha szeretnénk több jogosultságot hozzáadni, így kicsit könnyebb lesz
        // később.
        if user.admin {
            Ok(HashSet::from([Permission {
                name: "admin".to_owned(),
            }]))
        } else {
            Ok(HashSet::new())
        }
    }
}
