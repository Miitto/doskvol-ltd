#[cfg(feature = "server")]
use diesel::prelude::*;

use super::users::User;

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(belongs_to(User, foreign_key = dm_id)))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::crews))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Crew {
    pub id: i32,
    pub name: String,
    pub specialty: types::CrewSpecialty,
    pub dm_id: types::UserId,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::crews))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct NewCrew {
    pub name: String,
    pub specialty: types::CrewSpecialty,
    pub dm_id: types::UserId,
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable, Insertable)
)]
#[cfg_attr(feature = "server", diesel(belongs_to(User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Crew)))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::crew_members))]
#[cfg_attr(feature = "server", diesel(primary_key(user_id, crew_id)))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct CrewMember {
    pub user_id: types::UserId,
    pub crew_id: types::CrewId,
    pub display_name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(belongs_to(Crew)))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::crew_invites))]
#[cfg_attr(feature = "server", diesel(primary_key(code)))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct CrewInvite {
    pub code: String,
    pub crew_id: types::CrewId,
    pub used: i32,
    pub max_uses: i32,
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::crew_invites))]
pub struct NewCrewInvite {
    pub code: String,
    pub crew_id: types::CrewId,
    pub max_uses: i32,
}
