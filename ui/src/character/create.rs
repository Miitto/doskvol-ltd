use dioxus::prelude::*;

use crate::elements::Dialog;

#[component]
pub fn CreateCharacter(
    crew_id: usize,
    on_create: EventHandler<api::CharacterCreate>,
    open: Signal<bool>,
) -> Element {
    let mut name = use_signal(String::new);
    let mut class = use_signal(|| types::Class::Cutter);

    rsx! {
        Dialog {
            open,
            form {
                onsubmit: move |e| {
                    e.prevent_default();
                    if name().is_empty() {
                        return;
                    }
                    let char = api::CharacterCreate {
                        crew_id,
                        player_id: 0,
                        name: name(),
                        class: class()
                    };

                    on_create.call(char);
                },
                class: "flex flex-col gap-4",
                input {
                    class: "bg-input border border-border text-input-foreground",
                    placeholder: "Character Name",
                    onchange: move |e| {
                        name.set(e.value().clone());
                    }
                }
                select {
                    onchange: move |e| {
                        if let Ok(c) =  std::convert::TryInto::<types::Class>::try_into(e.value().as_str()) {
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
            }
        }
    }
}
