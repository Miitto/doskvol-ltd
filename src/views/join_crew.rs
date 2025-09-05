use dioxus::prelude::*;

use crate::elements::ErrorMessage;

#[component]
pub fn JoinCrew(code: ReadOnlySignal<String>) -> Element {
    #[allow(clippy::redundant_closure)]
    let mut code = use_signal(|| code());
    let mut name = use_signal(String::default);

    let mut error = use_signal(|| None as Option<String>);

    let nav = use_navigator();

    rsx! {
        div { class: "flex flex-col gap-4 p-4",
            h1 { class: "text-3xl font-bold mb-4", "Join a Crew" }

            form {
                class: "flex flex-col gap-4",
                onsubmit: move |e| async move {
                    e.prevent_default();
                    let code = code();
                    let name = name();
                    if code.is_empty() {
                        error.set(Some("Join code cannot be empty".into()));
                        return;
                    }
                    if name.is_empty() {
                        error.set(Some("Name cannot be empty".into()));
                        return;
                    }
                    let res = api::crew::join(code, name).await;
                    match res {
                        Ok(crew) => {
                            nav.push(crate::Route::Crew { id: crew.id });
                        }
                        Err(err) => {
                            error.set(Some(err.to_string()));
                        }
                    }
                },

                input {
                    class: "bg-input text-input-foreground rounded p-2",
                    value: "{code}",
                    placeholder: "Join Code",
                    oninput: move |e| {
                        code.set(e.value());
                    },
                }

                input {
                    class: "bg-input text-input-foreground rounded p-2",
                    value: "{name}",
                    placeholder: "Your Display Name",
                    oninput: move |e| {
                        name.set(e.value());
                    },
                }

                if let Some(err) = error.as_ref() {
                    ErrorMessage { class: "ml-1", "{err}" }
                }

                div { class: "flex flex-row justify-end items-center",
                    button { class: "bg-primary text-primary-foreground rounded-lg py-2 px-4 cursor-pointer",
                        "Join"
                    }
                }
            }
        }
    }
}
