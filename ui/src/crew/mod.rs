use dioxus::prelude::*;

mod create;
pub use create::CreateCrew;

use crate::character::CreateCharacter;

#[component]
pub fn Crew<R: 'static + Clone + PartialEq + Routable>(
    crew: types::Crew,
    to_character_page: Callback<types::CrewId, R>,
) -> Element {
    let id = crew.id;
    let mut crew_characters =
        use_server_future(move || async move { api::get_crew_characters(id).await.unwrap() })?;

    let mut open_create_character = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-col gap-4 p-4",
        if let Some(crew_characters) = crew_characters() {
            div { class: "flex flex-col grow gap-2",
                for character in crew_characters {
                    Link {
                        to: to_character_page.call(character.id),
                        h2 { "{character.name}" }
                        p { "Class: {character.class}" }
                    }
                }
            }
        }
            div { class: "flex flex-row justify-end",
                button {
                    class: "p-2 bg-primary text-primary-foreground rounded-lg cursor-pointer",
                    onclick: move |_| {
                        open_create_character.set(true);
                    },
                    "Create Character"
                }
            }
        }

        CreateCharacter { crew_id: crew.id,
            on_create: move |new_character| async move {
                let res = api::create_character(new_character).await;
                if let Err(err) = res {
                    tracing::error!("Failed to create crew: {:?}", err);
                } else {
                    crew_characters.restart();
                }
            },

            open: open_create_character
        }
    }
}
