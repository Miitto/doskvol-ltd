use dioxus::prelude::*;

use crate::elements::Dialog;

#[component]
pub fn CreateCrew(on_create: EventHandler<(api::NewCrew, String)>, open: Signal<bool>) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut specialty = use_signal(|| types::CrewSpecialty::Assassins);

    let mut dm_name = use_signal(String::default);

    let currentUser = use_context::<crate::Auth>();
    let currentUser = use_memo(move || {
        currentUser
            .username()
            .expect("User should be authenticated")
    });

    rsx! {
        Dialog {
            open,
            form {
                class: "flex flex-col gap-4",
                onsubmit: move |e| {
                    e.prevent_default();
                    let new_crew = api::NewCrew {
                        name: name(),
                        dm_id: currentUser(),
                        specialty: specialty()
                    };
                    if name().is_empty() {
                        return;
                    }
                    on_create.call((new_crew, dm_name()));
                    open.set(false);
                },
                input {
                    class: "border p-2 rounded w-full bg-input text-input-foreground",
                    value: name,
                    onchange: move |e| name.set(e.value().clone()),
                }
                select {
                    onchange: move |e| {
                        let value = e.value();
                        if let Ok(s) = value.parse() {
                            specialty.set(s);
                        }
                    },
                    for s in types::CrewSpecialty::ALL {
                        option {
                            value: "{s}",
                            selected: s == specialty(),
                            "{s}"
                        }
                    }
                }
                input {
                    class: "border p-2 rounded w-full bg-input text-input-foreground",
                    placeholder: "Name to display for DM",
                    value: dm_name,
                    onchange: move |e| dm_name.set(e.value()),
                }
                div { class: "flex flex-row justify-end gap-4",
                    button {
                        class: "p-2 bg-secondary text-secondary-foreground rounded-lg",
                        onclick: move |_| {
                            open.set(false);
                        },
                        "Cancel"
                    }
                    button {
                        class: "p-2 bg-primary text-primary-foreground rounded-lg",
                        r#type: "submit",
                        "Create Crew"
                    }
                }
            }
        }
    }
}
