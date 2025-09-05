#[cfg(feature = "server")]
use diesel::prelude::*;
use dioxus::prelude::{server_fn::error::NoCustomError, *};

use crate::db;

#[cfg(feature = "server")]
fn is_own_character(char_id: types::CharacterId, username: &str) -> bool {
    use crate::db::schema::characters::dsl;
    let mut conn = db::connect();

    dsl::characters
        .find(char_id)
        .filter(dsl::user_id.eq(username))
        .select(db::models::Character::as_select())
        .first::<db::models::Character>(&mut conn)
        .is_ok()
}

#[data::cfg_server("character/set_traits")]
pub async fn set_traits(
    id: types::CharacterId,
    heritage: types::Heritage,
    background: types::Background,
    vice: types::Vice,
) -> Result<(), ServerFnError> {
    use db::schema::characters::dsl;
    let user: crate::User = extract().await?;

    if !is_own_character(id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }

    let mut conn = db::connect();

    diesel::update(dsl::characters.find(id))
        .set((
            dsl::heritage.eq(heritage.to_string()),
            dsl::background.eq(background.to_string()),
            dsl::vice.eq(vice.to_string()),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character traits: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/set_look")]
pub async fn set_look(id: types::CharacterId, look: String) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::characters::dsl;

    let mut conn = db::connect();

    diesel::update(dsl::characters.find(id))
        .set(dsl::look.eq(look))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character look: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/set_stress_trauma_healing_armor")]
pub async fn set_stress_truama_healing_armor(
    id: types::CharacterId,
    stress: u8,
    trauma: u8,
    healing: u8,
    armor: u8,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::characters::dsl;

    let mut conn = db::connect();

    diesel::update(dsl::characters.find(id))
        .set((
            dsl::stress.eq(stress as i32),
            dsl::trauma.eq(trauma as i32),
            dsl::healing.eq(healing as i32),
            dsl::armor.eq(armor as i32),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character stress/trauma: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/set_harm")]
pub async fn set_harm(id: types::CharacterId, harm: types::Harm) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::character_harm::dsl;

    let mut conn = db::connect();

    diesel::update(dsl::character_harm.find(id))
        .set((
            dsl::harm_1_1.eq(&harm.0[0]),
            dsl::harm_1_2.eq(&harm.0[1]),
            dsl::harm_2_1.eq(&harm.1[0]),
            dsl::harm_2_2.eq(&harm.1[1]),
            dsl::harm_3.eq(harm.2),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character harm: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/set_description")]
pub async fn set_description(id: types::CharacterId, notes: String) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::characters::dsl;

    let mut conn = db::connect();

    diesel::update(dsl::characters.find(id))
        .set(dsl::notes.eq(notes))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character description: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/add_ability")]
pub async fn add_ability(
    character_id: types::CharacterId,
    name: String,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::character_abilities;

    let mut conn = db::connect();

    let new_character_ability = db::models::NewCharacterAbility { character_id, name };

    diesel::insert_into(character_abilities::table)
        .values(&new_character_ability)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to add ability to character: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to add ability".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/remove_ability")]
pub async fn remove_ability(
    character_id: types::CharacterId,
    name: String,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::character_abilities::dsl;

    let mut conn = db::connect();

    diesel::delete(
        dsl::character_abilities
            .filter(dsl::name.eq(name))
            .filter(dsl::character_id.eq(character_id)),
    )
    .execute(&mut conn)
    .map_err(|e| {
        tracing::error!("Failed to remove ability from character: {e}");
        ServerFnError::<NoCustomError>::ServerError("Failed to remove ability".to_string())
    })?;

    Ok(())
}

#[data::cfg_server("character/add_contact")]
pub async fn add_contact(
    character_id: types::CharacterId,
    name: String,
    friend: bool,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::character_contacts;

    let mut conn = db::connect();

    let new_character_contact = db::models::NewCharacterContact {
        character_id,
        name,
        friend,
    };

    diesel::insert_into(character_contacts::table)
        .values(&new_character_contact)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to add contact to character: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to add contact".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/remove_contact")]
pub async fn remove_contact(
    character_id: types::CharacterId,
    name: String,
    friend: bool,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::character_contacts::dsl;

    let mut conn = db::connect();

    diesel::delete(
        dsl::character_contacts
            .filter(dsl::name.eq(name))
            .filter(dsl::character_id.eq(character_id))
            .filter(dsl::friend.eq(friend)),
    )
    .execute(&mut conn)
    .map_err(|e| {
        tracing::error!("Failed to remove contact from character: {e}");
        ServerFnError::<NoCustomError>::ServerError("Failed to remove contact".to_string())
    })?;

    Ok(())
}

#[data::cfg_server("character/add_class_item")]
pub async fn add_class_item(
    character_id: types::CharacterId,
    name: String,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::character_class_items;

    let mut conn = db::connect();

    let new_character_class_item = db::models::NewCharacterClassItem { character_id, name };

    diesel::insert_into(character_class_items::table)
        .values(&new_character_class_item)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to add class item to character: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to add class item".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/remove_class_item")]
pub async fn remove_class_item(
    character_id: types::CharacterId,
    name: String,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::character_class_items::dsl;

    let mut conn = db::connect();

    diesel::delete(
        dsl::character_class_items
            .filter(dsl::name.eq(name))
            .filter(dsl::character_id.eq(character_id)),
    )
    .execute(&mut conn)
    .map_err(|e| {
        tracing::error!("Failed to remove class item from character: {e}");
        ServerFnError::<NoCustomError>::ServerError("Failed to remove class item".to_string())
    })?;

    Ok(())
}

#[data::cfg_server("character/set_coin_stash")]
pub async fn set_coin_stash(
    character_id: types::CharacterId,
    coin: u8,
    stash: u8,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::characters::dsl;

    let mut conn = db::connect();

    diesel::update(dsl::characters.find(character_id))
        .set((dsl::coin.eq(coin as i32), dsl::stash.eq(stash as i32)))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character coin stash: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/set_xp")]
pub async fn set_xp(character_id: types::CharacterId, xp: types::XP) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::character_xp::dsl;

    let mut conn = db::connect();

    diesel::update(dsl::character_xp.find(character_id))
        .set((
            dsl::playbook.eq(xp.playbook as i32),
            dsl::insight.eq(xp.insight as i32),
            dsl::prowess.eq(xp.prowess as i32),
            dsl::resolve.eq(xp.resolve as i32),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character xp: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/set_dots")]
pub async fn set_dots(
    character_id: types::CharacterId,
    dots: types::Dots,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::character_dots::dsl;

    let mut conn = db::connect();

    diesel::update(dsl::character_dots.find(character_id))
        .set((
            dsl::hunt.eq(dots.hunt as i32),
            dsl::study.eq(dots.study as i32),
            dsl::survey.eq(dots.survey as i32),
            dsl::tinker.eq(dots.tinker as i32),
            dsl::finesse.eq(dots.finesse as i32),
            dsl::prowl.eq(dots.prowl as i32),
            dsl::skirmish.eq(dots.skirmish as i32),
            dsl::wreck.eq(dots.wreck as i32),
            dsl::attune.eq(dots.attune as i32),
            dsl::command.eq(dots.command as i32),
            dsl::consort.eq(dots.consort as i32),
            dsl::sway.eq(dots.sway as i32),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character dots: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/set_load")]
pub async fn set_load(
    character_id: types::CharacterId,
    load: Option<types::Load>,
) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::characters::dsl;

    let mut conn = db::connect();

    let load = load.map(|l| match l {
        types::Load::Light => 0,
        types::Load::Medium => 1,
        types::Load::Heavy => 2,
    });

    diesel::update(dsl::characters.find(character_id))
        .set(dsl::load.eq(load))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character load: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("character/set_items")]
pub async fn set_items(character_id: types::CharacterId, items: u16) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_own_character(character_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Character not found".to_string(),
        ));
    }
    use db::schema::characters::dsl;

    let mut conn = db::connect();

    diesel::update(dsl::characters.find(character_id))
        .set(dsl::items.eq(items as i32))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update character items: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to update character".to_string())
        })?;

    Ok(())
}
