use dioxus::prelude::*;

#[component]
pub fn Crew(id: types::CrewId) -> Element {
    let crew = use_server_future(move || async move { api::crew::get_crew(id).await.unwrap() })?;

    rsx! {
        if let Some(crew) = crew() {
            ui::Crew {
                crew,
                to_character_page: move |character_id| crate::Route::Character {
                    id: character_id,
                },
            }
        }
    }
}
