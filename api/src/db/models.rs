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
    pub id: types::CharacterId,
    pub user_id: types::UserId,
    pub crew_id: types::CrewId,
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
    pub stash: i32,
    pub coin: i32,
    pub load: Option<i32>,
    pub items: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::characters))]
pub struct NewCharacter {
    pub name: String,
    pub crew_id: types::CrewId,
    pub user_id: types::UserId,
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
    pub character_id: types::CharacterId,
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
    pub character_id: types::CharacterId,
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
    pub character_id: types::CharacterId,
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
    pub character_id: types::CharacterId,
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
    pub character_id: types::CharacterId,
    pub playbook: i32,
    pub insight: i32,
    pub prowess: i32,
    pub resolve: i32,
}

impl From<CharacterXp> for types::XP {
    fn from(xp: CharacterXp) -> Self {
        types::XP {
            playbook: xp.playbook as u8,
            insight: xp.insight as u8,
            prowess: xp.prowess as u8,
            resolve: xp.resolve as u8,
        }
    }
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_dots))]
#[cfg_attr(feature = "server", diesel(belongs_to(Character)))]
#[cfg_attr(feature = "server", diesel(primary_key(character_id)))]
pub struct CharacterDots {
    pub character_id: types::CharacterId,
    pub hunt: i32,
    pub study: i32,
    pub survey: i32,
    pub tinker: i32,
    pub finesse: i32,
    pub prowl: i32,
    pub skirmish: i32,
    pub wreck: i32,
    pub attune: i32,
    pub command: i32,
    pub consort: i32,
    pub sway: i32,
}

impl From<CharacterDots> for types::Dots {
    fn from(dots: CharacterDots) -> Self {
        types::Dots {
            hunt: dots.hunt as u8,
            study: dots.study as u8,
            survey: dots.survey as u8,
            tinker: dots.tinker as u8,
            finesse: dots.finesse as u8,
            prowl: dots.prowl as u8,
            skirmish: dots.skirmish as u8,
            wreck: dots.wreck as u8,
            attune: dots.attune as u8,
            command: dots.command as u8,
            consort: dots.consort as u8,
            sway: dots.sway as u8,
        }
    }
}
