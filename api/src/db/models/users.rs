#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg_attr(feature = "server", derive(Queryable, Selectable, Identifiable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::users))]
#[cfg_attr(feature = "server", diesel(primary_key(username)))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct User {
    pub username: types::UserId,
    pub totp_secret: String,
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::users))]
pub struct NewUser {
    pub username: types::UserId,
    pub totp_secret: String,
}

#[cfg_attr(feature = "server", derive(Queryable, Selectable, Identifiable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::sessions))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Session {
    pub id: i32,
    pub user_id: types::UserId,
    pub token: String,
    pub name: Option<String>,
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::sessions))]
pub struct NewSession {
    pub user_id: types::UserId,
    pub token: String,
    pub name: Option<String>,
}
