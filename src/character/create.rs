use dioxus::prelude::*;

use crate::elements::Dialog;

#[component]
pub fn CreateCharacter(
    crew_id: types::CrewId,
    on_create: EventHandler<api::NewCharacter>,
    open: Signal<bool>,
) -> Element {
    let mut name = use_signal(String::new);
    let mut class = use_signal(|| types::Class::Cutter);

    let currentUser = use_context::<crate::Auth>();
    let currentUser = use_memo(move || {
        currentUser
            .username()
            .expect("User should be authenticated")
    });

    rsx! {
        Dialog { open, close_on_click: true,
            form {
                onsubmit: move |e| async move {
                    e.prevent_default();
                    if name().is_empty() {
                        return;
                    }
                    let char = api::NewCharacter {
                        crew_id,
                        user_id: currentUser(),
                        name: name(),
                        class: class(),
                    };
                    open.set(false);
                    on_create.call(char);
                },
                class: "flex flex-col gap-4",
                input {
                    class: "bg-input p-2 rounded text-input-foreground",
                    placeholder: "Character Name",
                    onchange: move |e| {
                        name.set(e.value().clone());
                    },
                }
                select {
                    class: "p-2",
                    onchange: move |e| {
                        if let Ok(c) = std::convert::TryInto::<
                            types::Class,
                        >::try_into(e.value().as_str()) {
                            class.set(c);
                        }
                    },
                    option { value: "cutter", "Cutter" }
                    option { value: "hound", "Hound" }
                    option { value: "leech", "Leech" }
                    option { value: "lurk", "Lurk" }
                    option { value: "slide", "Slide" }
                    option { value: "spider", "Spider" }
                    option { value: "whisper", "Whisper" }
                }

                div { class: "flex justify-between items-center gap-4",
                    button {
                        class: "bg-secondary text-secondary-foreground px-4 py-2 rounded",
                        onclick: move |e| {
                            e.prevent_default();
                            open.set(false);
                        },
                        "Cancel"
                    }
                    button { class: "bg-primary text-primary-foreground px-4 py-2 rounded",
                        "Create Character"
                    }
                }
            }
        }
    }
}
