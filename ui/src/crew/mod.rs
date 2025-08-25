use dioxus::prelude::*;

mod create;
pub use create::CreateCrew;

#[component]
pub fn Crew<R: 'static + Clone + PartialEq + Routable>(
    crew: types::Crew,
    to_character_page: Callback<usize, R>,
) -> Element {
    let id = crew.id;
    let crew_characters =
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
                    class: "p-2 bg-primary text-primary-foreground rounded-lg",
                    onclick: move |_| {
                        open_create_character.set(true);
                    },
                    "Create Character"
                }
            }
        }
    }
}
