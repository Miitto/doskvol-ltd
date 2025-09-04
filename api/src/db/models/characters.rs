#[cfg(feature = "server")]
use diesel::prelude::*;

use super::{crews::Crew, users::User};

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

pub(crate) struct IntoCharacter<
    A: Iterator<Item = CharacterAbility>,
    C: Iterator<Item = CharacterContact>,
    CI: Iterator<Item = CharacterClassItem>,
> {
    pub(crate) character: Character,
    pub(crate) harm: CharacterHarm,
    pub(crate) abilities: A,
    pub(crate) contacts: C,
    pub(crate) class_items: CI,
    pub(crate) xp: CharacterXp,
    pub(crate) dots: CharacterDots,
}

impl<
        A: Iterator<Item = CharacterAbility>,
        C: Iterator<Item = CharacterContact>,
        CI: Iterator<Item = CharacterClassItem>,
    > From<IntoCharacter<A, C, CI>> for types::Character
{
    fn from(
        IntoCharacter {
            character,
            harm,
            abilities,
            contacts,
            class_items,
            xp,
            dots,
        }: IntoCharacter<A, C, CI>,
    ) -> Self {
        let abilities = abilities.map(|a| a.name).collect();
        let class_items = class_items.map(|ci| ci.name).collect();

        let contacts: (Vec<CharacterContact>, Vec<CharacterContact>) =
            contacts.partition(|c| c.friend);

        let contacts = types::Contacts {
            friends: contacts.0.into_iter().map(|c| c.name).collect(),
            rivals: contacts.1.into_iter().map(|c| c.name).collect(),
        };

        let load = character.load.map(|load| match load {
            0 => types::Load::Light,
            1 => types::Load::Medium,
            2 => types::Load::Heavy,
            _ => {
                tracing::error!(
                    "Character ({}) has invalid load value: {}",
                    character.id,
                    load
                );
                types::Load::Light
            }
        });

        types::Character {
            id: character.id,
            user_id: character.user_id,
            crew_id: character.crew_id,
            name: character.name,
            look: types::Description::new(character.look),
            heritage: character.heritage,
            background: character.background,
            vice: character.vice,
            stress: character.stress as u8,
            trauma: types::TraumaFlags::from_bits_truncate(character.trauma as u8),
            harm: types::Harm(
                [harm.harm_1_1, harm.harm_1_2],
                [harm.harm_2_1, harm.harm_2_2],
                harm.harm_3,
            ),
            healing: character.healing as u8,
            armor: types::ArmorFlags::from_bits_truncate(character.armor as u8),
            notes: types::Description::new(character.notes),
            class: character.class,
            abilities,
            contacts,
            class_items,
            stash: character.stash as u8,
            coin: character.coin as u8,
            xp: xp.into(),
            dots: dots.into(),
            load,
            items: types::Items::from_bits_truncate(character.items as u16),
        }
    }
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
    derive(Queryable, Selectable, Associations, Identifiable, Insertable)
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

impl CharacterHarm {
    pub fn new(character_id: types::CharacterId) -> Self {
        Self {
            character_id,
            harm_1_1: String::new(),
            harm_1_2: String::new(),
            harm_2_1: String::new(),
            harm_2_2: String::new(),
            harm_3: String::new(),
        }
    }
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

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_abilities))]
pub struct NewCharacterAbility {
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

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_contacts))]
pub struct NewCharacterContact {
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

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::db::schema::character_class_items))]
pub struct NewCharacterClassItem {
    pub character_id: types::CharacterId,
    pub name: String,
}

#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Associations, Identifiable, Insertable)
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

impl CharacterXp {
    pub fn new(character_id: types::CharacterId) -> Self {
        Self {
            character_id,
            playbook: 0,
            insight: 0,
            prowess: 0,
            resolve: 0,
        }
    }
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
    derive(Queryable, Selectable, Associations, Identifiable, Insertable)
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

impl CharacterDots {
    pub fn new(character_id: types::CharacterId) -> Self {
        Self {
            character_id,
            hunt: 0,
            study: 0,
            survey: 0,
            tinker: 0,
            finesse: 0,
            prowl: 0,
            skirmish: 0,
            wreck: 0,
            attune: 0,
            command: 0,
            consort: 0,
            sway: 0,
        }
    }
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
