use dioxus::prelude::*;

#[component]
pub fn Character(id: types::CharacterId) -> Element {
    let character =
        use_server_future(move || async move { api::character::get(id).await.unwrap() })?;

    let auth: crate::Auth = use_context();

    let readonly = use_memo(move || {
        character().is_none_or(|c| auth.username().is_none_or(|u| u != c.user_id))
    });

    rsx! {
        if let Some(character) = character() {
            crate::character::Character { character, readonly: readonly() }
        } else {
            p { "No character found." }
        }
    }
}
