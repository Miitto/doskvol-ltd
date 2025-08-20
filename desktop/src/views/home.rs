use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let character = use_server_future(async move || api::get_character(0).await.unwrap())?;

    rsx! {
        if let Some(character) = character() {
            ui::Character { character, readonly: false }
        } else {
            p { "No character found." }
        }
    }
}
