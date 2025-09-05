use crate::db;

#[cfg(feature = "server")]
use diesel::prelude::*;

use dioxus::prelude::{server_fn::error::NoCustomError, *};
use totp_rs::{Algorithm, Secret, TOTP};

pub mod session;

#[server]
pub async fn generate_totp_secret(name: String) -> Result<TOTP, ServerFnError> {
    let secret = Secret::generate_secret().to_bytes()?;
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret,
        Some("Doskvol-Ltd".to_string()),
        name,
    )?;

    Ok(totp)
}

#[data::cfg_server("auth/current_user")]
pub async fn get_current_user() -> Result<Option<types::User>, ServerFnError> {
    let user: Option<types::User> = extract().await.ok().map(|u: crate::User| types::User {
        username: u.username,
    });

    Ok(user)
}

#[data::cfg_server("auth/login")]
pub async fn login(username: String, code: String) -> Result<types::User, ServerFnError> {
    let mut conn = db::connect();

    let user: db::models::User = db::schema::users::table
        .filter(db::schema::users::username.eq(username))
        .select(db::models::User::as_select())
        .first(&mut conn)
        .map_err(|e| {
            tracing::info!("Failed to find user: {e}");
            ServerFnError::<NoCustomError>::Request("User not found".to_string())
        })?;

    let secret = totp_rs::Secret::Encoded(user.totp_secret);
    let secret = secret.to_raw()?;

    let totp = totp_rs::TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        secret.to_bytes()?,
        Some("Doskvol-Ltd".to_string()),
        user.username.clone(),
    )?;

    let user = if cfg!(debug_assertions) || totp.check_current(&code)? {
        types::User {
            username: user.username,
        }
    } else {
        tracing::info!("Invalid TOTP code for user: {}", user.username);
        return Err(ServerFnError::<NoCustomError>::Request(
            "Invalid code".to_string(),
        ));
    };

    if let Err(()) = session::set_current_user(&user.username).await {
        tracing::error!("Failed to create session for user: {}", user.username);
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Failed to create session".to_string(),
        ));
    }

    tracing::info!("Logged in user: {}", user.username);

    Ok(user)
}

#[data::cfg_server("auth/register")]
pub async fn register(username: String, totp_secret: String) -> Result<types::User, ServerFnError> {
    let mut conn = db::connect();

    let new_user = db::models::NewUser {
        username: username.clone(),
        totp_secret,
    };

    let user: db::models::User = diesel::insert_into(db::schema::users::table)
        .values(&new_user)
        .returning(db::models::User::as_returning())
        .get_result(&mut conn)
        .map_err(|e| {
            dioxus::logger::tracing::info!("Failed to create user: {e}");
            ServerFnError::<NoCustomError>::Request("Failed to create user".to_string())
        })?;

    let user = types::User {
        username: user.username,
    };

    if let Err(()) = session::set_current_user(&user.username).await {
        tracing::error!("Failed to create session for new user");
    }

    Ok(user)
}

#[server]
pub async fn check_username(username: String) -> Result<Option<String>, ServerFnError> {
    let mut conn = db::connect();

    if username.is_empty() {
        return Ok(Some("Usernames cannot be empty".to_string()));
    }

    if username.contains(':') {
        return Ok(Some("Usernames cannot contain colons':'".to_string()));
    }

    let count: i64 = db::schema::users::table
        .filter(db::schema::users::username.eq(username))
        .count()
        .get_result(&mut conn)
        .map_err(|e| {
            dioxus::logger::tracing::error!("Failed to check username: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to check username".to_string())
        })?;

    if count > 0 {
        Ok(Some("Username already in use".to_string()))
    } else {
        Ok(None)
    }
}

#[data::cfg_server("auth/logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    if let Err(()) = session::clear_current_user().await {
        tracing::error!("Failed to clear session on logout");
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Failed to clear session".to_string(),
        ));
    }

    Ok(())
}
