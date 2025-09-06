use dioxus::prelude::*;

#[component]
pub fn Crew(id: types::CrewId) -> Element {
    let crew = use_resource(move || async move { api::crew::get_crew(id).await });

    rsx! {
        match crew() {
            Some(Ok(crew)) => rsx! {
                crate::crew::Crew {
                    crew,
                    to_character_page: move |character_id| crate::Route::Character {
                        id: character_id,
                    },
                }
            },
            Some(Err(e)) => rsx! { "Error loading crew: {e}" },
            None => rsx! { "Loading..." }
        }
    }
}
