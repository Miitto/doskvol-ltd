use dioxus::prelude::*;
use types::description::{Description as DT, Segment};

#[component]
pub fn Description<T: std::fmt::Display + 'static + PartialEq>(desc: DT<T>) -> Element {
    let segments = desc.to_segments();

    rsx! {
        p {
            for seg in segments {
                if let Segment::Text(text) = seg {
                    span { "{text}" }
                } else if let Segment::Italic(text) = seg {
                    span { class: "italic", "{text}" }
                } else if let Segment::Bold(text) = seg {
                    span { class: "font-bold", "{text}" }
                } else if let Segment::Newline = seg {
                    br {}
                } else {

                }
            }
        }
    }
}

#[component]
pub fn DescriptionEdit(
    desc: DT<String>,
    readonly: Option<bool>,
    on_change: EventHandler<DT<String>>,
) -> Element {
    let readonly = readonly.unwrap_or(true);

    let mut editing = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col",
            if editing() {
                textarea {
                    class: "w-full h-32 p-2 border border-border rounded",
                    autofocus: true,
                    value: desc.to_string(),
                    oninput: move |e| {
                        let new_desc = DT::new(e.value());
                        on_change.call(new_desc);
                    },
                    onblur: move |_| {
                        editing.set(false);
                    },
                    onmounted: move |cx| async move {
                        _ = cx.set_focus(true).await;
                    },
                }
            } else {
                Description { desc }
                if !readonly {
                    div { class: "flex justify-end",
                        button {
                            class: "cursor-pointer bg-primary text-primary-foreground rounded-lg p-2 px-4",
                            onclick: move |_| {
                                editing.set(true);
                            },
                            "Edit"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Dialog(
    open: Signal<bool>,
    children: Element,
    close_on_click: ReadOnlySignal<Option<bool>>,
) -> Element {
    let close_on_click = use_memo(move || close_on_click().unwrap_or(false));

    let display = if open() { "flex" } else { "hidden" };

    rsx! {
        dialog {
            open,
            class: "bg-transparent fixed top-0 left-0 w-screen h-screen {display} flex-col justify-center items-center z-10 backdrop-blur",
            onclick: move |_| {
                if close_on_click() {
                    open.set(false);
                }
            },
            div {
                class: "bg-background w-max h-max max-w-[calc(100vw_-_2rem)] max-h-[calc(100vh_-_2rem)] text-foreground p-4 rounded-lg shadow-lg border border-border",
                onclick: move |e| {
                    e.stop_propagation();
                },
                {children}
            }
        }
    }
}

#[component]
pub fn ErrorMessage(children: Element, class: ReadOnlySignal<Option<String>>) -> Element {
    let class = use_memo(move || class().unwrap_or_default());
    rsx! {
        p { class: "text-destructive brightness-200 {class}", {children} }
    }
}
