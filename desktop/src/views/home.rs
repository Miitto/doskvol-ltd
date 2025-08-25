use dioxus::prelude::*;
use ui::Crew;

#[component]
pub fn Home() -> Element {
    let crews = use_server_future(|| async move { api::get_all_crews().await.unwrap() })?;

    rsx! {
        div { class: "flex flex-col gap-4 p-4",
            h1 { class: "text-3xl font-bold mb-4", "Crews" }
                if let Some(crews) = crews() {
                    div {
                        class: "flex flex-col gap-2",
                        for crew in crews {
                            Link {
                                to: crate::Route::Crew { id: crew.id },
                                "{crew.name}"
                            }
                        }
                    }
                }
        }
    }
}
