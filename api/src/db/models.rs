use diesel::prelude::*;

#[cfg_attr(feature = "server", derive(Queryable, Selectable, Identifiable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::users))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::users))]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}

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
    pub specialty: String,
    pub dm_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::crews))]
pub struct NewCrew {
    pub name: String,
    pub specialty: types::CrewSpecialty,
    pub dm_id: i32,
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
    pub user_id: i32,
    pub crew_id: i32,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(belongs_to(User)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Crew)))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::characters))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct Character {
    pub id: i32,
    pub user_id: i32,
    pub crew_id: i32,
    pub name: String,
    pub look: String,
    pub heritage: types::Heritage,
    pub background: types::Background,
    pub vice: types::Vice,
    pub stress: i32,
    pub trauma: i32,
    pub healing: i32,
    pub armor: i32,
    pub notes: String,
    pub class: types::Class,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::characters))]
pub struct NewCharacter {
    pub name: String,
    pub crew_id: i32,
    pub user_id: i32,
    pub class: types::Class,
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_harm))]
#[cfg_attr(feature = "server", diesel(belongs_to(Character)))]
#[cfg_attr(feature = "server", diesel(primary_key(character_id)))]
pub struct CharacterHarm {
    pub character_id: i32,
    pub harm_1_1: String,
    pub harm_1_2: String,
    pub harm_2_1: String,
    pub harm_2_2: String,
    pub harm_3: String,
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_abilities))]
#[cfg_attr(feature = "server", diesel(belongs_to(Character)))]
pub struct CharacterAbility {
    pub id: i32,
    pub character_id: i32,
    pub name: String,
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_contacts))]
#[cfg_attr(feature = "server", diesel(belongs_to(Character)))]
pub struct CharacterContact {
    pub id: i32,
    pub character_id: i32,
    pub name: String,
    pub friend: bool,
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_class_items))]
#[cfg_attr(feature = "server", diesel(belongs_to(Character)))]
pub struct CharacterClassItem {
    pub id: i32,
    pub character_id: i32,
    pub name: String,
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_items))]
#[cfg_attr(feature = "server", diesel(belongs_to(Character)))]
pub struct CharacterItem {
    pub id: i32,
    pub character_id: i32,
    pub name: String,
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_xp))]
#[cfg_attr(feature = "server", diesel(belongs_to(Character)))]
#[cfg_attr(feature = "server", diesel(primary_key(character_id)))]
pub struct CharacterXp {
    pub character_id: i32,
    pub playbook: i32,
    pub insight: i32,
    pub prowess: i32,
    pub resolve: i32,
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_dots))]
#[cfg_attr(feature = "server", diesel(belongs_to(Character)))]
#[cfg_attr(feature = "server", diesel(primary_key(character_id)))]
pub struct CharacterXp {
    pub character_id: i32,
    pub hunt: i32,
    pub study: i32,
    pub survey: i32,
    pub tinker: i32,
    pub finesse: i32,
    pub prowl: i32,
    pub skirmish: i32,
    pub wreck: i32,
    pub arcane: i32,
    pub command: i32,
    pub consort: i32,
    pub sway: i32,
}
