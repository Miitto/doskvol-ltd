//! This crate contains all shared UI for the workspace.
use dioxus::prelude::*;

mod character;
pub use character::Character;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn Tailwind() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
    }
}
