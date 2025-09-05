use dioxus::prelude::*;

mod center;
mod left;
mod right;

mod create;
pub use create::CreateCharacter;

use center::Center;
use left::Left;
use right::Right;

#[component]
pub fn Character(character: types::Character, readonly: ReadOnlySignal<Option<bool>>) -> Element {
    let character = use_signal(|| character);

    let readonly = use_memo(move || readonly().unwrap_or(true));

    rsx! {
        div { class: "flex flex-col lg:flex-row h-full",
            Left { readonly, character }
            div { class: "bg-border min-h-px h-px w-full lg:w-px lg:h-full" }
            Center { character, readonly }
            div { class: "bg-border min-h-px h-px w-full lg:w-px lg:h-full" }
            Right { character, readonly }
        }
    }
}
