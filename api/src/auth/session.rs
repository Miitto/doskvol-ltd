use dioxus::prelude::*;

pub const SESSION_COOKIE_NAME: &str = "session_token";

pub enum Auth {
    User(crate::User),
    Anon,
}

impl Auth {
    pub fn ok(self) -> Option<crate::User> {
        match self {
            Auth::User(u) => Some(u),
            Auth::Anon => None,
        }
    }
}

#[cfg(feature = "server")]
#[async_trait::async_trait]
impl FromServerContext for Auth {
    type Rejection = ServerFnError<server_fn::error::NoCustomError>;
    async fn from_request(req: &DioxusServerContext) -> Result<Self, Self::Rejection> {
        use axum_extra::extract::cookie::CookieJar;
        use diesel::prelude::*;

        let headers = &req.request_parts().headers;

        let jar = CookieJar::from_headers(headers);

        let auth = jar.get(SESSION_COOKIE_NAME).ok_or_else(|| {
            ServerFnError::<server_fn::error::NoCustomError>::Request("Not authenticated".into())
        })?;

        if auth.value().is_empty() || auth.value() == "deleted" {
            tracing::debug!("Empty session cookie");
            return Ok(Auth::Anon);
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

            delete_session_cookie();

            return Ok(Auth::Anon);
        };

        crate::db::schema::users::table
            .find(session.user_id)
            .select(crate::db::models::User::as_select())
            .first(&mut conn)
            .inspect(|i| tracing::info!("Authenticated user: {}", i.username))
            .inspect_err(|e| tracing::error!("Failed to load user: {e}"))
            .map_err(|_| {
                ServerFnError::<server_fn::error::NoCustomError>::Request(
                    "Not authenticated".into(),
                )
            })
            .map(Auth::User)
    }
}

#[cfg(feature = "server")]
#[async_trait::async_trait]
impl FromServerContext for crate::User {
    type Rejection = ServerFnError<server_fn::error::NoCustomError>;

    async fn from_request(req: &DioxusServerContext) -> Result<Self, Self::Rejection> {
        let auth: Auth = FromServerContext::from_request(req).await?;

        auth.ok().ok_or_else(|| {
            ServerFnError::<server_fn::error::NoCustomError>::Request("Not authenticated".into())
        })
    }
}

#[cfg(feature = "server")]
pub async fn set_current_user(user: &types::UserId) -> Result<(), ()> {
    use diesel::prelude::*;

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

    set_session_cookie(&new_session.token);

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

    delete_session_cookie();

    Ok(())
}

#[cfg(feature = "server")]
fn set_session_cookie(value: &str) {
    let context = server_context();

    let cookie = cookie::Cookie::build((SESSION_COOKIE_NAME, value))
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
            .append(http::header::SET_COOKIE, header_value);

        tracing::info!("Removed session cookie");
    } else {
        tracing::error!("Failed to remove session cookie");
    }
}

#[cfg(feature = "server")]
fn delete_session_cookie() {
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

        tracing::info!("Removed session cookie");
    } else {
        tracing::error!("Failed to remove session cookie");
    }
}
