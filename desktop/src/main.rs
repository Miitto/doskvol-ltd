use dioxus::prelude::*;

use views::{Character, Crew, Home, Login, Register};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopNavbar)]
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

    rsx! {
        div {
            class: "{linux}",
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
fn AuthRequired() -> Element {
    let auth: Signal<ui::Auth> = use_context();
    let nav = use_navigator();

    use_effect(move || {
        if auth().is_anon() {
            nav.replace(Route::Login {});
        }
    });

    let authed = use_memo(move || auth().is_authenticated());

    rsx! {
        if authed() {
            Outlet::<Route> {}
        }
    }
}
