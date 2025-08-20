use dioxus::prelude::*;

#[component]
pub fn Character(character: Signal<types::Character>) -> Element {
    let heritage = use_memo(move || character().heritage);
    let background = use_memo(move || character().background);
    let vice = use_memo(move || character().vice);

    let name = use_signal(|| character().name);
    let class = use_signal(|| character().class);
    let look = use_signal(|| character().look);
    let coin = use_memo(move || character().coin);
    let stash = use_memo(move || character().stash);

    rsx! {
        div { class: "flex flex-col md:flex-row h-full",
            // Left
            div {
                class: "flex flex-col gap-2 w-fit p-4",
                input { class: "text-4xl", value: "{name}" }
                input { value: "{look}" }

                hr {class: "my-2"}

                div { class: "grid grid-cols-2 gap-2 md:flex md:flex-row",
                    DropdownList { name: "Herritage", value: heritage, set: move |h| {
                        character.with_mut(|char| {
                            char.heritage = h;
                        });
                    },
                        for h in types::Heritage::ALL {
                            option {
                                value: "{h}",
                                "{h}"
                            }
                        }
                    }
                    DropdownList { name: "Background", value: background, set: move |b| {
                        character.with_mut(|char| {
                            char.background = b;
                        });
                    },
                        for b in types::Background::ALL {
                            option {
                                value: "{b}",
                                "{b}"
                            }
                        }
                    }
                }
                DropdownList { name: "Vice", value: vice, set: move |v| {
                    character.with_mut(|char| {
                        char.vice = v;
                    });
                },
                    for v in types::Vice::ALL {
                        option {
                            value: "{v}",
                            "{v}"
                        }
                    }
                }
            }
            div{class: "bg-border w-px h-full"}
            // Center
            div {
                class: "flex flex-col gap-2 w-fit p-4",
                h1 { class: "text-6xl", "{class}" }
            }
            // Right
            div {
                class: "flex flex-row gap-2 w-fit p-4 h-32 shrink ml-auto",
                div {
                    class: "flex flex-col w-fit h-fit",
                    span { "Stash "}
                    span { "Coin" }
                    Coin { coin: coin, set: move |c| {
                        character.with_mut(|char| {char.coin = c;});
                    } }
                }
                    Stash { stash: stash, set: move |s| {
                        character.with_mut(|c| {c.stash = s;});
                    } }
            }
        }
    }
}

#[component]
fn DropdownList<T: std::fmt::Display + Clone + PartialEq + From<String> + 'static>(
    name: &'static str,
    value: ReadOnlySignal<T>,
    set: EventHandler<T>,
    children: Element,
) -> Element {
    rsx! {
        label { class: "flex gap-2 items-center",
            "{name}"
            select { value: value.to_string(), onchange: move |e| {
                set(e.value().into());
            },  {children}}
        }
    }
}

#[component]
fn Stash(stash: ReadOnlySignal<u8>, set: EventHandler<u8>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-1 w-full h-fit",
        for y in 0..4 {
            div {
                class: "flex flex-row gap-1 w-full h-fit",
            for x in 0..10 {
                button { class: "h-5 aspect-square",
                    onclick: move |_| {
                        let current = stash();
                        if current == (y * 10) + x + 1 {
                            set(0);
                        } else {
                            set((y * 10) + x + 1);
                        }
                    },
                    style: if stash() > (y * 10) + x {
                        "background-color: #EEE;"
                    } else {
                        "background-color: #999;"
                    }
                }
            }
            }
        }
        }
    }
}

#[component]
fn Coin(coin: ReadOnlySignal<u8>, set: EventHandler<u8>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-1 w-full h-max",
            for y in 0..2 {
                div {
                    class: "flex flex-row gap-1 w-full h-max",
                    for x in 0..2 {
                        button { class: "h-5 aspect-square",
                            onclick: move |_| {
                                let current = coin();
                                if current == (y * 2) + x + 1 {
                                    set(0);
                                } else {
                                    set((y * 2) + x + 1);
                                }
                            },
                            style: if coin() > (y * 2) + x {
                                "background-color: #EEE;"
                            } else {
                                "background-color: #999;"
                            }
                        }
                    }
                }
            }
        }
    }
}
