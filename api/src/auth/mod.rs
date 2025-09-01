use crate::db;
use diesel::prelude::*;
use dioxus::prelude::{server_fn::error::NoCustomError, *};
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Debug, PartialEq)]
pub enum TotpError {
    Url(totp_rs::TotpUrlError),
    Secret(totp_rs::SecretParseError),
}

impl From<totp_rs::TotpUrlError> for TotpError {
    fn from(err: totp_rs::TotpUrlError) -> Self {
        TotpError::Url(err)
    }
}

impl From<totp_rs::SecretParseError> for TotpError {
    fn from(err: totp_rs::SecretParseError) -> Self {
        TotpError::Secret(err)
    }
}

impl std::fmt::Display for TotpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TotpError::Url(e) => write!(f, "TOTP URL error: {e}"),
            TotpError::Secret(e) => write!(f, "TOTP Secret error: {e}"),
        }
    }
}

impl std::error::Error for TotpError {}

pub fn generate_totp_secret(name: String) -> Result<TOTP, TotpError> {
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

#[server(Login)]
pub async fn login(username: String, code: String) -> Result<types::User, ServerFnError> {
    let mut conn = db::connect();

    let user: db::models::User = db::schema::users::table
        .filter(db::schema::users::username.eq(username))
        .select(db::models::User::as_select())
        .first(&mut conn)
        .map_err(|e| {
            dioxus::logger::tracing::info!("Failed to find user: {e}");
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

    if totp.check_current(&code)? {
        Ok(types::User {
            username: user.username,
        })
    } else {
        Err(ServerFnError::<NoCustomError>::Request(
            "Invalid code".to_string(),
        ))
    }
}

#[server(Register)]
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

    Ok(types::User {
        username: user.username,
    })
}

#[server(CheckUsername)]
pub async fn check_username(username: String) -> Result<Option<String>, ServerFnError> {
    let mut conn = db::connect();

    let count: i64 = db::schema::users::table
        .filter(db::schema::users::username.eq(username))
        .count()
        .get_result(&mut conn)
        .map_err(|e| {
            dioxus::logger::tracing::error!("Failed to check username: {e}");
            ServerFnError::<NoCustomError>::ServerError("Failed to check username".to_string())
        })?;

    if count > 0 {
        Ok(Some("Username alrady in use".to_string()))
    } else {
        Ok(None)
    }
}
