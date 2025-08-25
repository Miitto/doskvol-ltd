use dioxus::prelude::*;

use crate::elements::Dialog;

#[component]
pub fn CreateCharacter(on_create: EventHandler<types::CharacterPreview>, open: Signal<bool>) -> Element {
    rsx! {
        Dialog {
            open,
            div {
                class: "flex flex-col gap-4",
                input {
                    class: "bg-input border border-border text-input-foreground",
                    placeholder: "Character Name",
                    onchange: move |e| {
                        let name = e.value().clone();
                        on_create.call(types::CharacterPreview {
                            id: 0,
                            name,
                        });
                    }
                }
            }
        }
    }
}
