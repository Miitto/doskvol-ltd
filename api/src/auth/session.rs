use dioxus::prelude::*;

pub const SESSION_COOKIE_NAME: &str = "session_token";

#[cfg(feature = "server")]
pub async fn get_current_user() -> Option<crate::db::models::User> {
    use axum_extra::extract::cookie::CookieJar;
    use diesel::prelude::*;

    let headers: http::HeaderMap = extract().await.ok()?;

    let jar = CookieJar::from_headers(&headers);

    let auth = jar.get(SESSION_COOKIE_NAME)?;

    if auth.value().is_empty() || auth.value() == "deleted" {
        tracing::debug!("Empty session cookie");
        return None;
    }

    let mut conn = crate::db::connect();

    let session: crate::db::models::Session = if let Ok(s) = crate::db::schema::sessions::table
        .filter(crate::db::schema::sessions::token.eq(auth.value()))
        .select(crate::db::models::Session::as_select())
        .first(&mut conn)
    {
        s
    } else {
        tracing::info!("Invalid session token: {}", auth.value());
        return None;
    };

    crate::db::schema::users::table
        .find(session.user_id)
        .select(crate::db::models::User::as_select())
        .first(&mut conn)
        .ok()
        .inspect(|i| tracing::info!("Authenticated user: {}", i.username))
}

#[cfg(feature = "server")]
pub async fn set_current_user(user: &types::UserId) -> Result<(), ()> {
    use axum_extra::extract::cookie::Cookie;
    use diesel::prelude::*;
    use http::header::SET_COOKIE;

    let token = nanoid::nanoid!(25);

    let mut conn = crate::db::connect();

    let new_session = crate::db::models::NewSession {
        user_id: user.clone(),
        token,
        name: None,
    };

    diesel::insert_into(crate::db::schema::sessions::table)
        .values(&new_session)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Failed to create session: {e}");
        })?;

    let context = server_context();

    let cookie = Cookie::build((SESSION_COOKIE_NAME, new_session.token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(cookie::SameSite::Strict)
        .permanent()
        .build();

    if let Ok(header_value) = cookie.encoded().to_string().parse() {
        context
            .response_parts_mut()
            .headers
            .append(SET_COOKIE, header_value);
    } else {
        tracing::error!("Failed to set session cookie for user {}", user);
    }

    Ok(())
}

#[cfg(feature = "server")]
pub async fn clear_current_user() -> Result<(), ()> {
    use axum_extra::extract::cookie::CookieJar;
    use diesel::prelude::*;
    let mut conn = crate::db::connect();

    let headers: http::HeaderMap = extract()
        .await
        .map_err(|e| tracing::error!("Failed to get header map when removing session: {e}"))?;

    let jar = CookieJar::from_headers(&headers);

    let auth = jar.get(SESSION_COOKIE_NAME).ok_or_else(|| {
        tracing::info!("No session cookie found when removing session");
    })?;

    diesel::delete(
        crate::db::schema::sessions::table
            .filter(crate::db::schema::sessions::token.eq(auth.value())),
    )
    .execute(&mut conn)
    .map_err(|e| {
        tracing::error!("Failed to delete session: {e}");
    })?;

    let context = server_context();

    let cookie = cookie::Cookie::build(SESSION_COOKIE_NAME)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(cookie::SameSite::Strict)
        .removal()
        .build();

    if let Ok(header_value) = cookie.encoded().to_string().parse() {
        context
            .response_parts_mut()
            .headers
            .append(http::header::SET_COOKIE, header_value);
    } else {
        tracing::error!("Failed to remove session cookie");
    }

    Ok(())
}
