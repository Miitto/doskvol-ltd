use dioxus::prelude::*;

#[component]
pub fn CountBtn(
    this: u8,
    total: u8,
    readonly: Option<bool>,
    set: EventHandler<u8>,
    class: Option<String>,
) -> Element {
    let class = class.unwrap_or_default();
    let color = if this <= total {
        "bg-neutral-300"
    } else {
        "bg-neutral-500"
    };

    let hover_class = if readonly.unwrap_or(false) {
        "cursor-not-allowed"
    } else {
        "hover:bg-neutral-400 group-hover:bg-neutral-400 cursor-pointer"
    };

    rsx! {
        button {
            data: "{this}",
            class: "h-5 aspect-square {hover_class} {color} {class}",
            onclick: move |_| {
                if readonly.unwrap_or(false) {
                    return;
                }
                if this != total {
                    set(this);
                } else {
                    set(0);
                }
            },
        }
    }
}

#[component]
pub fn ItemChecked(
    checked: bool,
    onclick: EventHandler<bool>,
    readonly: ReadOnlySignal<bool>,
) -> Element {
    rsx! {
        CountBtn {
            this: if checked { 1 } else { 2 },
            total: 1,
            readonly: readonly(),
            set: move |_| {
                onclick.call(!checked);
            },
        }
    }
}
