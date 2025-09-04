use dioxus::prelude::*;

#[cfg(feature = "desktop")]
use server_fn::client::reqwest::ReqwestClient;

pub struct Client;

#[cfg(feature = "desktop")]
impl<T: std::fmt::Display> server_fn::client::Client<T> for Client {
    type Request = <ReqwestClient as server_fn::client::Client<T>>::Request;

    type Response = <ReqwestClient as server_fn::client::Client<T>>::Response;

    async fn send(mut req: Self::Request) -> Result<Self::Response, ServerFnError<T>> {
        use std::{collections::HashMap, io::Write as _};

        let home_dir = std::env::home_dir().ok_or_else(|| {
            tracing::error!("Failed to get home dir");
            ServerFnError::Request("Could not determine home directory".into())
        })?;

        let desktop_file = home_dir.join(".doskvol-ltd").join("cookies.txt");

        if desktop_file.exists() {
            let content = std::fs::read_to_string(&desktop_file).map_err(|e| {
                tracing::error!("Could not read cookies file: {e}");
                ServerFnError::Request(format!("Could not read session file: {e}"))
            })?;

            let headers = req.headers_mut();

            let cookies = headers
                .entry("Cookie")
                .or_insert_with(|| http::HeaderValue::from_static(""));

            let s = cookies.to_str().unwrap_or_default().to_string();

            let new_cookies = if s.is_empty() {
                content
            } else {
                format!("{s}; {content}")
            };

            *cookies = new_cookies.parse().map_err(|e| {
                tracing::error!("Failed to parse cookies: {e}");
                ServerFnError::Request(format!("Could not parse cookies: {e}"))
            })?;
        }

        let res = ReqwestClient::send(req)
            .await
            .inspect_err(|e| tracing::error!("Request error: {e}"))?;

        let headers = res.headers();

        let set_cookies = headers
            .get_all("set-cookie")
            .iter()
            .filter_map(|value| value.to_str().ok())
            .filter_map(|cookie| cookie.split(';').next())
            .map(|cookie| cookie.split_once('=').unwrap_or((cookie, "")))
            .collect::<HashMap<_, _>>();

        let cookies_str = std::fs::read_to_string(&desktop_file).unwrap_or_default();
        let mut cookies = cookies_str
            .split("; ")
            .filter_map(|s| s.split_once('='))
            .collect::<std::collections::HashMap<_, _>>();

        for (key, value) in set_cookies {
            cookies.insert(key, value);
        }

        let cookies = cookies
            .into_iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("; ");

        std::fs::create_dir_all(home_dir.join(".doskvol-ltd")).map_err(|e| {
            tracing::error!("Could not create session directory: {e}");
            ServerFnError::Request(format!("Could not create session directory: {e}"))
        })?;

        let mut file = std::fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&desktop_file)
            .map_err(|e| {
                tracing::error!("Couldn't open cookies file: {e}");
                ServerFnError::Request(format!("Could not open cookies file: {e}"))
            })?;

        file.write_all(cookies.as_bytes()).map_err(|e| {
            tracing::error!("Failed to write cookies file: {e}");
            ServerFnError::Request(format!("Could not write session file: {e}"))
        })?;

        Ok(res)
    }
}
