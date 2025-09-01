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
pub enum Auth {
    Authenticated {
        username: types::UserId,
    },
    #[default]
    Anon,
}

impl Auth {
    pub fn is_authenticated(&self) -> bool {
        matches!(self, Auth::Authenticated { .. })
    }

    pub fn is_anon(&self) -> bool {
        matches!(self, Auth::Anon)
    }
}

#[component]
pub fn AuthProvider(children: Element) -> Element {
    let auth = use_signal(Auth::default);

    use_context_provider(|| auth);

    rsx! {
        {children}
    }
}
