#[cfg(feature = "server")]
use diesel::prelude::*;

use dioxus::prelude::{server_fn::error::NoCustomError, *};

use crate::{db, CrewInvite, CrewMember};
#[cfg(feature = "server")]
use db::schema::*;

#[cfg(feature = "server")]
pub(crate) fn is_in_crew(crew_id: types::CrewId, user_id: &str) -> bool {
    let mut conn = db::connect();

    crew_members::table
        .filter(
            crew_members::crew_id
                .eq(crew_id)
                .and(crew_members::user_id.eq(user_id)),
        )
        .select(crew_members::crew_id)
        .first::<types::CrewId>(&mut conn)
        .is_ok()
}

#[data::cfg_server("crew/get_characters")]
pub async fn get_crew_characters(
    crew_id: types::CrewId,
) -> Result<Vec<types::CharacterPreview>, ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_in_crew(crew_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Not a member of this crew".to_string(),
        ));
    }

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
        .map(|m| {
            let player_name = db::schema::crew_members::table
                .filter(
                    db::schema::crew_members::crew_id
                        .eq(crew_id)
                        .and(db::schema::crew_members::user_id.eq(&m.user_id)),
                )
                .select(db::schema::crew_members::display_name)
                .first::<String>(&mut conn)
                .unwrap_or_else(|_| "Failed to load player name".to_string());

            types::CharacterPreview {
                id: m.id,
                name: m.name,
                class: m.class,
                player_id: m.user_id,
                player_name,
                crew_id,
            }
        })
        .collect())
}

#[data::cfg_server("crew/get")]
pub async fn get_crew(id: types::CrewId) -> Result<types::Crew, ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_in_crew(id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Not a member of this crew".to_string(),
        ));
    }
    let mut conn = db::connect();

    let crew = db::schema::crews::table
        .find(id)
        .select(db::models::Crew::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to find crew: {e}");
            ServerFnError::<NoCustomError>::Request("Crew not found".to_string())
        })?;

    Ok(types::Crew {
        id: crew.id,
        name: crew.name,
        specialty: crew.specialty,
        dm_id: crew.dm_id,
    })
}

#[data::cfg_server("crew/my_crews")]
pub async fn get_my_crews() -> Result<Vec<types::CrewPreview>, ServerFnError> {
    let user: crate::User = extract().await?;

    let mut conn = db::connect();

    let crews: Vec<db::models::Crew> = crews::table
        .inner_join(crew_members::table)
        .filter(crew_members::user_id.eq(&user.username))
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

#[data::cfg_server("crew/create")]
pub async fn create_crew(
    crew: db::models::NewCrew,
    dm_name: String,
) -> Result<types::Crew, ServerFnError> {
    use db::schema::crews::dsl::*;
    let user: crate::User = extract().await?;

    if user.username != crew.dm_id {
        return Err(ServerFnError::<NoCustomError>::Request(
            "DM ID must match the current user".to_string(),
        ));
    }

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

#[data::cfg_server("crew/get_player_display_name")]
pub async fn get_player_display_name(
    crew_id: types::CrewId,
    user_id: String,
) -> Result<String, ServerFnError> {
    let user: crate::User = extract().await?;

    if !is_in_crew(crew_id, &user.username) {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Not a member of this crew".to_string(),
        ));
    }

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

#[data::cfg_server("crew/create_invite")]
pub async fn create_invite(
    crew_id: types::CrewId,
    max_uses: i32,
) -> Result<CrewInvite, ServerFnError> {
    const CODE_CHARS: [char; 62] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let user: crate::User = extract().await?;

    let mut conn = db::connect();

    let is_dm = crews::table
        .filter(crews::id.eq(crew_id).and(crews::dm_id.eq(&user.username)))
        .select(crews::id)
        .first::<types::CrewId>(&mut conn)
        .is_ok();

    if !is_dm {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Only the DM can create invites".to_string(),
        ));
    }

    let code: String = nanoid::nanoid!(6, &CODE_CHARS);

    let invite = db::models::NewCrewInvite {
        code,
        crew_id,
        max_uses,
    };

    let invite = diesel::insert_into(crew_invites::table)
        .values(&invite)
        .returning(db::models::CrewInvite::as_returning())
        .get_result(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to insert new crew invite: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to create crew invite".to_string())
        })?;

    Ok(CrewInvite {
        code: invite.code,
        crew_id: invite.crew_id,
        used: invite.used,
        max_uses: invite.max_uses,
    })
}

#[data::cfg_server("crew/join")]
pub async fn join(code: String, name: String) -> Result<types::Crew, ServerFnError<String>> {
    let user: crate::User = extract()
        .await
        .map_err(|e: ServerFnError| ServerFnError::Request(e.to_string()))?;

    let mut conn = db::connect();

    let invite: db::models::CrewInvite = crew_invites::table
        .filter(crew_invites::code.eq(&code))
        .select(db::models::CrewInvite::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to find crew invite: {e}");
            ServerFnError::WrappedServerError("Invalid invite code".to_string())
        })?;

    let already_member = crew_members::table
        .filter(
            crew_members::crew_id
                .eq(invite.crew_id)
                .and(crew_members::user_id.eq(&user.username)),
        )
        .select(crew_members::user_id)
        .first::<types::UserId>(&mut conn)
        .is_ok();

    if already_member {
        return Err(ServerFnError::WrappedServerError(
            "You are already a member of this crew".to_string(),
        ));
    }

    if invite.used >= invite.max_uses {
        return Err(ServerFnError::<String>::WrappedServerError(
            "Invite code has reached its maximum uses".to_string(),
        ));
    }

    diesel::insert_into(crew_members::table)
        .values(&db::models::CrewMember {
            crew_id: invite.crew_id,
            user_id: user.username,
            display_name: name,
        })
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to insert new crew member: {e}");
            ServerFnError::<String>::ServerError("Failed to join crew".to_string())
        })?;

    diesel::update(crew_invites::table.filter(crew_invites::code.eq(&code)))
        .set(crew_invites::used.eq(crew_invites::used + 1))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to update crew invite usage: {e}");
            ServerFnError::<String>::ServerError("Failed to join crew".to_string())
        })?;

    diesel::delete(db::schema::crew_invites::table)
        .filter(crew_invites::used.ge(crew_invites::max_uses))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to delete expired invites: {e}");
            ServerFnError::<String>::ServerError("Failed to join crew".to_string())
        })?;

    let crew: db::models::Crew = crews::table
        .find(invite.crew_id)
        .select(db::models::Crew::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to find crew for invite: {e}");
            ServerFnError::<String>::ServerError("Corrupt invite data".to_string())
        })?;

    Ok(types::Crew {
        id: crew.id,
        name: crew.name,
        specialty: crew.specialty,
        dm_id: crew.dm_id,
    })
}

#[data::cfg_server("crew/delete_invite")]
pub async fn delete_invite(code: String) -> Result<(), ServerFnError> {
    let user: crate::User = extract().await?;

    let mut conn = db::connect();

    let invite: db::models::CrewInvite = crew_invites::table
        .filter(crew_invites::code.eq(&code))
        .select(db::models::CrewInvite::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to find crew invite: {e}");
            ServerFnError::<NoCustomError>::Request("Invite not found".to_string())
        })?;

    let is_dm = crews::table
        .filter(
            crews::id
                .eq(invite.crew_id)
                .and(crews::dm_id.eq(&user.username)),
        )
        .select(crews::id)
        .first::<types::CrewId>(&mut conn)
        .is_ok();

    if !is_dm {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Only the DM can delete invites".to_string(),
        ));
    }

    diesel::delete(crew_invites::table.filter(crew_invites::code.eq(&code)))
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to delete crew invite: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to delete invite".to_string())
        })?;

    Ok(())
}

#[data::cfg_server("crew/get_invites")]
pub async fn get_invites(crew_id: types::CrewId) -> Result<Vec<CrewInvite>, ServerFnError> {
    let user: crate::User = extract().await?;

    let mut conn = db::connect();

    let is_dm = crews::table
        .filter(crews::id.eq(crew_id).and(crews::dm_id.eq(&user.username)))
        .select(crews::id)
        .first::<types::CrewId>(&mut conn)
        .is_ok();

    if !is_dm {
        return Err(ServerFnError::<NoCustomError>::Request(
            "Only the DM can view invites".to_string(),
        ));
    }

    let invites: Vec<db::models::CrewInvite> = crew_invites::table
        .filter(crew_invites::crew_id.eq(crew_id))
        .select(db::models::CrewInvite::as_select())
        .load(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to load crew invites: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to load invites".to_string())
        })?;

    Ok(invites)
}
