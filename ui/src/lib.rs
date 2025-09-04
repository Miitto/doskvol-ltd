//! This crate contains all shared UI for the workspace.
use dioxus::prelude::*;

pub mod common;
pub mod elements;

mod character;
pub use character::Character;

pub mod crew;
pub use crew::Crew;

pub mod auth;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn Tailwind() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum AuthState {
    Authenticated {
        username: types::UserId,
    },
    #[default]
    Anon,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Auth {
    server_user: Resource<Option<types::User>>,
    client: Memo<AuthState>,
    _is_authenticated: Memo<bool>,
    _is_anon: Memo<bool>,
    _username: Memo<Option<types::UserId>>,
}

impl Auth {
    pub fn new() -> Result<Self, RenderError> {
        let server_user = use_server_future(|| async {
            api::auth::get_current_user()
                .await
                .inspect_err(|e| tracing::error!("Error getting auth: {e}"))
                .ok()
                .flatten()
        })?;
        let client = use_memo(move || {
            if let Some(Some(user)) = server_user() {
                AuthState::Authenticated {
                    username: user.username.clone(),
                }
            } else {
                AuthState::Anon
            }
        });

        Ok(Self {
            server_user,
            client,
            _username: use_memo(move || {
                if let AuthState::Authenticated { username } = client() {
                    Some(username.clone())
                } else {
                    None
                }
            }),
            _is_authenticated: use_memo(move || {
                matches!(client(), AuthState::Authenticated { .. })
            }),
            _is_anon: use_memo(move || matches!(client(), AuthState::Anon)),
        })
    }

    pub fn refresh(&mut self) {
        self.server_user.restart();
    }

    pub fn username(&self) -> Option<types::UserId> {
        let u = self._username;
        u()
    }

    pub fn is_authenticated(&self) -> bool {
        let a = self._is_authenticated;
        a()
    }

    pub fn is_anon(&self) -> bool {
        let a = self._is_anon;
        a()
    }
}

#[component]
pub fn AuthProvider(children: Element) -> Element {
    let auth = Auth::new()?;
    use_context_provider(|| auth);

    rsx! {
        {children}
    }
}
