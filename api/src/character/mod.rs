mod updates;
use dioxus::prelude::server_fn::error::NoCustomError;
pub use updates::*;

#[cfg(feature = "server")]
use diesel::prelude::*;
use dioxus::prelude::*;

use crate::db;
#[cfg(feature = "server")]
use crate::db::schema::*;

#[data::cfg_server("character/get")]
pub async fn get(id: types::CharacterId) -> Result<types::Character, ServerFnError> {
    let user = crate::auth::session::get_current_user()
        .await
        .ok_or_else(|| ServerFnError::<NoCustomError>::Request("Not authenticated".to_string()))?;

    let mut conn = db::connect();

    let character: db::models::Character = characters::table
        .find(id)
        .select(db::models::Character::as_select())
        .first(&mut conn)
        .map_err(|e| {
            dioxus::logger::tracing::info!("Failed to find character: {e}");
            ServerFnError::<NoCustomError>::Request("Character not found".to_string())
        })?;

    if !crate::crew::is_in_crew(character.crew_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }

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

#[data::cfg_server("character/create")]
pub async fn create(
    character: db::models::NewCharacter,
) -> Result<types::Character, ServerFnError> {
    let user = crate::auth::session::get_current_user()
        .await
        .ok_or_else(|| ServerFnError::<NoCustomError>::Request("Not authenticated".to_string()))?;

    if !crate::crew::is_in_crew(character.crew_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Cannot create character in crew you are not a member of".to_string(),
        ));
    }

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
