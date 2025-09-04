use dioxus::{logger::tracing, prelude::*};
use ui::crew::CreateCrew;

#[component]
pub fn Home() -> Element {
    let mut crews = use_server_future(|| async move { api::crew::get_my_crews().await })?;
    let mut create_crew_open = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-4 p-4",
            h1 { class: "text-3xl font-bold mb-4", "Crews" }
            if let Some(Ok(crews)) = crews() {
                div {
                    class: "flex flex-col gap-2 grow",
                    for crew in crews {
                        Link {
                            class: "hover:bg-input hover:text-input-foreground p-2 rounded-lg",
                            to: crate::Route::Crew { id: crew.id },
                            "{crew.name}"
                        }
                    }
                }
            }

            div {class: "flex flex-row justify-end",
                button { class: "p-2 bg-primary text-primary-foreground rounded-lg",
                    onclick: move |_| {
                        create_crew_open.set(true);
                    },
                    "Create New Crew"
                }
            }
        }
        CreateCrew {
            open: create_crew_open,
            on_create: move |(new_crew, dm_name)| async move {
                let res = api::crew::create_crew(new_crew, dm_name).await;
                if let Err(err) = res {
                    tracing::error!("Failed to create crew: {:?}", err);
                } else {
                    crews.restart();
                }
            }
        }
    }
}
