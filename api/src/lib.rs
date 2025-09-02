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

mod character_updates;
pub use character_updates::*;

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
        .into_iter();

    let contacts = db::models::CharacterContact::belonging_to(&character)
        .select(db::models::CharacterContact::as_select())
        .load(&mut conn)
        .map_err(|e| {
            tracing::error!(
                "Failed to get contacts for charracter ({}): {e}",
                character.id
            );
            ServerFnError::<NoCustomError>::ServerError("Corrupt character data".to_string())
        })?
        .into_iter();

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
        .into_iter();

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

    let character = db::models::IntoCharacter {
        character,
        harm,
        abilities,
        class_items,
        contacts,
        xp,
        dots,
    }
    .into();

    Ok(character)
}

#[server(GetCrewCharacters)]
pub async fn get_crew_characters(
    crew_id: types::CrewId,
) -> Result<Vec<types::CharacterPreview>, ServerFnError> {
    let mut conn = db::connect();

    let members: Vec<db::models::Character> = db::schema::characters::table
        .filter(db::schema::characters::crew_id.eq(crew_id))
        .select(db::models::Character::as_select())
        .get_results(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to get crew members for crew ({}): {e}", crew_id);
            ServerFnError::<NoCustomError>::ServerError("Failed to load crew members".to_string())
        })?;

    Ok(members
        .into_iter()
        .map(|m| types::CharacterPreview {
            id: m.id,
            name: m.name,
            class: m.class,
            player_id: m.user_id,
            crew_id,
        })
        .collect())
}

#[server(GetCrew)]
pub async fn get_crew(id: types::CrewId) -> Result<types::Crew, ServerFnError> {
    let mut conn = db::connect();

    let user = db::schema::crews::table
        .find(id)
        .select(db::models::Crew::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to find crew: {e}");
            ServerFnError::<NoCustomError>::Request("Crew not found".to_string())
        })?;

    Ok(types::Crew {
        id: user.id,
        name: user.name,
        specialty: user.specialty,
        dm_id: user.dm_id,
    })
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
pub async fn create_crew(
    crew: db::models::NewCrew,
    dm_name: String,
) -> Result<types::Crew, ServerFnError> {
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

    diesel::insert_into(crew_members::table)
        .values(&db::models::CrewMember {
            crew_id: crew.id,
            user_id: crew.dm_id.clone(),
            display_name: dm_name,
        })
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to insert new crew member: {e}");
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
    let mut conn = db::connect();

    let character = diesel::insert_into(db::schema::characters::table)
        .values(&character)
        .returning(db::models::Character::as_returning())
        .get_result::<db::models::Character>(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to insert new character: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to create character".to_string())
        })?;

    let harm = db::models::CharacterHarm::new(character.id);
    diesel::insert_into(db::schema::character_harm::table)
        .values(&harm)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to insert new character harm: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to create character".to_string())
        })?;

    let xp = db::models::CharacterXp::new(character.id);
    diesel::insert_into(db::schema::character_xp::table)
        .values(&xp)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to insert new character xp: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to create character".to_string())
        })?;

    let dots = db::models::CharacterDots::new(character.id);
    diesel::insert_into(db::schema::character_dots::table)
        .values(&dots)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to insert new character dots: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to create character".to_string())
        })?;

    Ok(db::models::IntoCharacter {
        character,
        harm,
        abilities: std::iter::empty(),
        class_items: std::iter::empty(),
        contacts: std::iter::empty(),
        xp,
        dots,
    }
    .into())
}

#[server(GetPlayerDisplayName)]
pub async fn get_player_display_name(
    crew_id: types::CrewId,
    user_id: String,
) -> Result<String, ServerFnError> {
    let mut conn = db::connect();

    let member: db::models::CrewMember = crew_members::table
        .filter(
            crew_members::crew_id
                .eq(crew_id)
                .and(crew_members::user_id.eq(user_id)),
        )
        .select(db::models::CrewMember::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to find crew member: {e}");
            ServerFnError::<NoCustomError>::Request("Crew member not found".to_string())
        })?;

    Ok(member.display_name)
}
