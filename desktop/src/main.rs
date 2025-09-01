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
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ui::AuthProvider {
            Router::<Route> {}
        }
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        ui::Tailwind {}
        Outlet::<Route> {}
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

    rsx! {
        if auth().is_authenticated() {
            Outlet::<Route> {}
        }
    }
}
