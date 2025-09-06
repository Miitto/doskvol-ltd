use dioxus::prelude::*;

#[component]
pub fn Character(id: types::CharacterId) -> Element {
    let character = use_resource(move || {
        let id = id;
        async move { api::character::get(id).await }
    });

    let auth: crate::Auth = use_context();

    let readonly = use_memo(move || {
        character().is_none_or(|c| {
            c.is_err() || c.is_ok_and(|c| auth.username().is_none_or(|u| u != c.user_id))
        })
    });

    rsx! {
        match character() {
            Some(Ok(character)) => rsx! {
                crate::character::Character {
                    character,
                    readonly: readonly(),
                }
            },
            Some(Err(e)) => rsx! { "Error loading character: {e}" },
            None => rsx! { "Loading..." },
        }
    }
}
