use crate::crew::CreateCrew;
use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn Home() -> Element {
    let mut crews = use_server_future(|| async move { api::crew::get_my_crews().await })?;
    let mut create_crew_open = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-4 p-4",
            h1 { class: "text-3xl font-bold mb-4", "Crews" }
            if let Some(Ok(crews)) = crews() {
                div { class: "flex flex-col gap-2 grow",
                    for crew in crews {
                        Link {
                            class: "hover:bg-input hover:text-input-foreground p-2 rounded-lg",
                            to: crate::Route::Crew { id: crew.id },
                            div { class: "flex flex-row justify-between items-center",
                                "{crew.name}"
                                span { class: "italic", "{crew.dm_name}" }
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-row justify-between",
                button {
                    class: "p-2 bg-secondary text-secondary-foreground rounded-lg",
                    onclick: move |_| {
                        create_crew_open.set(true);
                    },
                    "Create New Crew"
                }

                Link {
                    class: "bg-primary text-primary-foreground rounded-lg p-2",
                    to: crate::Route::JoinCrew {code: "".into()},
                    "Join a Crew"
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
            },
        }
    }
}
