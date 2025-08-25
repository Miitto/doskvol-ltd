use dioxus::prelude::*;

use crate::elements::Dialog;

#[component]
pub fn CreateCrew(on_create: EventHandler<api::CrewCreate>, open: Signal<bool>) -> Element {
    let mut name = use_signal(|| "".to_string());

    rsx! {
        Dialog { 
            open,
            form {
                onsubmit: move |e| {
                    e.prevent_default();
                    let new_crew = api::CrewCreate {
                        name: name(),
                    };
                    if name().is_empty() {
                        return;
                    }
                    on_create.call(new_crew);
                    open.set(false);
                },
                input {
                    class: "border p-2 rounded w-full bg-input text-input-foreground",
                    value: name,
                    onchange: move |e| name.set(e.value().clone()),
                }
            }
        }
    }
}
