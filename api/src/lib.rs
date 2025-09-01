//! This crate contains all shared fullstack server functions.
use dioxus::prelude::{server_fn::error::NoCustomError, *};

#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg(feature = "server")]
use db::schema::*;

mod db;

pub mod auth;
pub use auth::{login, register};

pub use db::models::*;

#[cfg(feature = "server")]
fn launch_server() {
    tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(async {});
}

#[server(GetCharacter)]
pub async fn get_character(id: types::CharacterId) -> Result<types::Character, ServerFnError> {
    let mut conn = db::connect();

    let character: db::models::Character = characters::table
        .find(id)
        .select(db::models::Character::as_select())
        .first(&mut conn)
        .map_err(|e| {
            dioxus::logger::tracing::info!("Failed to find character: {e}");
            ServerFnError::<NoCustomError>::Request("Character not found".to_string())
        })?;

    let harm = db::models::CharacterHarm::belonging_to(&character)
        .select(db::models::CharacterHarm::as_select())
        .load(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to get harm for character ({}): {e}", character.id);
            ServerFnError::<NoCustomError>::ServerError("Corrupt character data".to_string())
        })?;

    if harm.len() != 1 {
        tracing::error!(
            "Character ({}) has {} harm entries, expected 1",
            character.id,
            harm.len()
        );
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Corrupt character data".to_string(),
        ));
    }

    let harm = harm.into_iter().next().unwrap();

    let abilities = db::models::CharacterAbility::belonging_to(&character)
        .select(db::models::CharacterAbility::as_select())
        .load(&mut conn)
        .map_err(|e| {
            tracing::error!(
                "Failed to get abilities for charracter ({}): {e}",
                character.id
            );
            ServerFnError::<NoCustomError>::ServerError("Corrupt character data".to_string())
        })?
        .into_iter()
        .map(|a| a.name);

    let contacts: (Vec<CharacterContact>, Vec<CharacterContact>) =
        db::models::CharacterContact::belonging_to(&character)
            .select(db::models::CharacterContact::as_select())
            .load(&mut conn)
            .map_err(|e| {
                tracing::error!(
                    "Failed to get contacts for charracter ({}): {e}",
                    character.id
                );
                ServerFnError::<NoCustomError>::ServerError("Corrupt character data".to_string())
            })?
            .into_iter()
            .partition(|a| a.friend);

    let contacts = types::Contacts {
        friends: contacts.0.into_iter().map(|c| c.name).collect(),
        rivals: contacts.1.into_iter().map(|c| c.name).collect(),
    };

    let class_items = db::models::CharacterClassItem::belonging_to(&character)
        .select(db::models::CharacterClassItem::as_select())
        .load(&mut conn)
        .map_err(|e| {
            tracing::error!(
                "Failed to get class items for charracter ({}): {e}",
                character.id
            );
            ServerFnError::<NoCustomError>::ServerError("Corrupt character data".to_string())
        })?
        .into_iter()
        .map(|c| c.name);

    let xp = db::models::CharacterXp::belonging_to(&character)
        .select(db::models::CharacterXp::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to get xp for character ({}): {e}", character.id);
            ServerFnError::<NoCustomError>::ServerError("Corrupt character data".to_string())
        })?;

    let dots = db::models::CharacterDots::belonging_to(&character)
        .select(db::models::CharacterDots::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to get dots for character ({}): {e}", character.id);
            ServerFnError::<NoCustomError>::ServerError("Corrupt character data".to_string())
        })?;

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

    let character = types::Character {
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
        abilities: abilities.collect(),
        contacts,
        class_items: class_items.collect(),
        stash: character.stash as u8,
        coin: character.coin as u8,
        xp: xp.into(),
        dots: dots.into(),
        load,
        items: types::Items::from_bits_truncate(character.items as u16),
    };

    Ok(character)
}

#[server(GetCrewCharacters)]
pub async fn get_crew_characters(
    crew_id: types::CrewId,
) -> Result<Vec<types::CharacterPreview>, ServerFnError> {
    todo!()
}

#[server(GetCrew)]
pub async fn get_crew(id: types::CrewId) -> Result<types::Crew, ServerFnError> {
    todo!()
}

#[server(GetAllCrews)]
pub async fn get_all_crews() -> Result<Vec<types::CrewPreview>, ServerFnError> {
    let mut conn = db::connect();

    let crews: Vec<db::models::Crew> = crews::table
        .select(db::models::Crew::as_select())
        .load(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to load crews: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to load crews".to_string())
        })?;

    let crews = crews
        .into_iter()
        .map(|c| {
            let dm_name = crew_members::table
                .filter(
                    crew_members::user_id
                        .eq(c.dm_id)
                        .and(crew_members::crew_id.eq(c.id)),
                )
                .select(CrewMember::as_select())
                .first(&mut conn)
                .map(|cm: CrewMember| cm.display_name)
                .unwrap_or_else(|_| "Failed to load DM name".to_string());

            let player_count = crew_members::table
                .filter(crew_members::crew_id.eq(c.id))
                .count()
                .get_result::<i64>(&mut conn)
                .unwrap_or(0) as usize;

            types::CrewPreview {
                id: c.id,
                name: c.name,
                specialty: c.specialty,
                dm_name,
                player_count,
            }
        })
        .collect();

    Ok(crews)
}

#[server(CreateCrew)]
pub async fn create_crew(crew: db::models::NewCrew) -> Result<types::Crew, ServerFnError> {
    use db::schema::crews::dsl::*;

    let mut conn = db::connect();

    let crew = diesel::insert_into(crews)
        .values(&crew)
        .returning(db::models::Crew::as_returning())
        .get_result(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to insert new crew: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to create crew".to_string())
        })?;

    Ok(types::Crew {
        id: crew.id,
        name: crew.name,
        specialty: crew.specialty,
        dm_id: crew.dm_id,
    })
}

#[server(CreateCharacter)]
pub async fn create_character(
    character: db::models::NewCharacter,
) -> Result<types::Character, ServerFnError> {
    todo!()
}
