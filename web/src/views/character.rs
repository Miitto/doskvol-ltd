use dioxus::prelude::*;

#[component]
pub fn Character(id: types::CharacterId) -> Element {
    let character =
        use_server_future(move || async move { api::character::get(id).await.unwrap() })?;

    rsx! {
        if let Some(character) = character() {
            ui::Character { character, readonly: false }
        } else {
            p { "No character found." }
        }
    }
}
