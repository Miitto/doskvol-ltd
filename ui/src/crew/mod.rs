use dioxus::prelude::*;

#[component]
pub fn Crew<R: 'static + Clone + PartialEq + Routable>(
    crew: types::Crew,
    to_character_page: Callback<usize, R>,
) -> Element {
    let id = crew.id;
    let crew_characters =
        use_server_future(move || async move { api::get_crew_characters(id).await.unwrap() })?;

    rsx! {
        if let Some(crew_characters) = crew_characters() {
            div { class: "flex flex-col",
                for character in crew_characters {
                    Link {
                        to: to_character_page.call(character.id),
                        h2 { "{character.name}" }
                        p { "Class: {character.class}" }
                    }
                }
            }
        }
    }
}
