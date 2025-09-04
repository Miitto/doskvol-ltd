use dioxus::prelude::*;

use views::{Character, Crew, Home, Login, Register};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopNavbar)]
    #[layout(AuthManager)]
        #[route("/login")]
        Login {},
        #[route("/register")]
        Register {},
        #[layout(AuthRequired)]
            #[route("/")]
            Home {},
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
        ui::AuthProvider {
            Router::<Route> {
                config: || {
                    RouterConfig::default()
                        .on_update(|state| {
                            dioxus::logger::tracing::trace!("Navigation to: {:?}", state.current());
                            None
                        })
                }
            }
        }
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    let linux = if cfg!(target_os = "linux") {
        "linux"
    } else {
        ""
    };

    let nav = use_navigator();

    rsx! {
        div {
            class: "{linux} flex flex-col",
            div {
                class: "border-b border-border",
                button {
                    class: "hover:underline w-fit p-2 rounded-lg",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    "Back"
                }
            }
            ui::Tailwind {}
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

    let auth: ui::Auth = use_context();
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
    let auth: ui::Auth = use_context();
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
