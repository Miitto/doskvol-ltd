use dioxus::prelude::*;

#[component]
pub fn CountBtn(this: u8, total: u8, readonly: Option<bool>, set: EventHandler<u8>) -> Element {
    let class = if this < total {
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
            class: "h-5 aspect-square {hover_class} {class}",
            onclick: move |_| {
                if readonly.unwrap_or(false) {
                    return;
                }
                if this + 1 != total {
                    set(this + 1);
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
            this: if checked { 0 } else { 1 },
            total: 1,
            readonly: readonly(),
            set: move |_| {
                onclick.call(!checked);
            },
        }
    }
}
