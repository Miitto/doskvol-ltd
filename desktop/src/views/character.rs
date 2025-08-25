use dioxus::prelude::*;

#[component]
pub fn Character(id: usize) -> Element {
    let character =
        use_server_future(move || async move { api::get_character(id).await.unwrap() })?;

    rsx! {
        if let Some(character) = character() {
            ui::Character { character, readonly: false }
        } else {
            p { "No character found." }
        }
    }
}
