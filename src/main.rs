use dioxus::prelude::*;

use views::{Character, Crew, Home, JoinCrew, Login, Register};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[layout(AuthManager)]
        #[route("/login")]
        Login {},
        #[route("/register")]
        Register {},
        #[layout(AuthRequired)]
            #[route("/")]
            Home {},
            #[route("/crew/join?:code")]
            JoinCrew {code: String},
            #[route("/crew/:id")]
            Crew { id: types::CrewId },
            #[route("/character/:id")]
            Character { id: types::CrewId },
        #[end_layout]
        #[route("/:..route")]
        PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        AuthProvider {
            Router::<Route> {
                config: || {
                    RouterConfig::default()
                        .on_update(|state| {
                            dioxus::logger::tracing::trace!("Navigation to: {:?}", state.current());
                            None
                        })
                },
            }
        }
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn Navbar() -> Element {
    let linux = if cfg!(target_os = "linux") {
        "linux"
    } else {
        ""
    };

    let nav = use_navigator();

    let mut auth: Auth = use_context();

    rsx! {
        div { class: "{linux} flex flex-col",
            div { class: "border-b border-border flex flex-row justify-between items-center",
                button {
                    class: "hover:underline w-fit p-2 rounded-lg cursor-pointer",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    "Back"
                }
                button {
                    class: "hover:underline w-fit p-2 rounded-lg cursor-pointer",
                    onclick: move |_| async move {
                        if auth.is_authenticated() {
                            if let Err(e) = api::auth::logout().await {
                                tracing::error!("Failed to log out: {e}");
                            }
                            auth.refresh();
                        } else {
                            nav.push(Route::Login {});
                        }
                    },
                    if auth.is_authenticated() { "Logout" } else { "Login" }
                }
            }
            Tailwind {}
            Outlet::<Route> {}
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "404 - Page not found" }
        Link { to: Route::Home {}, "Go home" }
    }
}

#[component]
fn AuthManager() -> Element {
    let mut redir_from = use_signal(|| None as Option<Route>);
    use_context_provider(|| redir_from);

    let auth: Auth = use_context();
    let mut is_authed = auth.is_authenticated();
    let nav = use_navigator();

    let route: Route = use_route();

    use_effect(move || {
        if auth.is_authenticated() {
            if !is_authed {
                let to = redir_from.peek().clone();
                redir_from.set(None);
                nav.replace(to.unwrap_or(Route::Home {}));
                is_authed = true;
            } else if matches!(route, Route::Login {} | Route::Register {}) {
                nav.replace(Route::Home {});
            }
        } else if !auth.is_authenticated() {
            is_authed = false;
        }
    });

    rsx! {
        Outlet::<Route> {}
    }
}

#[component]
fn AuthRequired() -> Element {
    let auth: Auth = use_context();
    let nav = use_navigator();

    let current: Route = use_route();
    let mut redir_from = use_context::<Signal<Option<Route>>>();

    use_effect(move || {
        dioxus::logger::tracing::trace!("Auth changed: {:?}", auth.is_authenticated());
        if auth.is_anon() {
            dioxus::logger::tracing::info!("User is not authenticated, redirecting to login");
            redir_from.set(Some(current.clone()));
            nav.replace(Route::Login {});
        }
    });

    let authed = use_memo(move || auth.is_authenticated());

    rsx! {
        if authed() {
            Outlet::<Route> {}
        }
    }
}

pub mod common;
pub mod elements;

mod character;

pub mod crew;

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
