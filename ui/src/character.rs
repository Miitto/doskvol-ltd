use dioxus::prelude::*;
use types::BitCount;

use crate::elements::Description;

#[component]
pub fn Character(character: types::Character, readonly: Option<bool>) -> Element {
    let character = use_signal(|| character);

    let editing = use_signal(|| !readonly.unwrap_or(true));

    rsx! {
        div { class: "flex flex-col lg:flex-row h-full",
            Left { editing, character }
            div { class: "bg-border h-px w-full md:w-px md:h-full" }
            Center { character }
            div { class: "bg-border h-px w-full md:w-px md:h-full" }
            Right { character, editing }
        }
    }
}

#[component]
fn Left(editing: ReadOnlySignal<bool>, mut character: Signal<types::Character>) -> Element {
    let name = use_signal(|| character().name);
    let look = use_signal(|| character().look);
    let heritage = use_memo(move || character().heritage);
    let background = use_memo(move || character().background);
    let vice = use_memo(move || character().vice);
    let stress = use_memo(move || character().stress);

    rsx! {
        div { class: "flex flex-col gap-2 w-fit p-4",
            h2 { class: "block text-4xl w-max", "{name}" }
            if editing() {
                input { value: "{look}" }
            } else {
                Description { desc: look() }
            }

            hr { class: "my-2" }

            div { class: "grid grid-cols-2 gap-2",
                DropdownList {
                    name: "Herritage",
                    value: "{heritage}",
                    readonly: !editing(),
                    set: move |h| {
                        character
                            .with_mut(|char| {
                                char.heritage = h;
                            });
                    },
                    for h in types::Heritage::ALL {
                        option { value: "{h}", selected: heritage() == h, "{h}" }
                    }
                }
                DropdownList {
                    name: "Background",
                    value: "{background}",
                    readonly: !editing(),
                    set: move |b| {
                        character
                            .with_mut(|char| {
                                char.background = b;
                            });
                    },
                    for b in types::Background::ALL {
                        option { value: "{b}", selected: background() == b, "{b}" }
                    }
                }
                DropdownList {
                    name: "Vice",
                    value: "{vice}",
                    readonly: !editing(),
                    set: move |v| {
                        character
                            .with_mut(|char| {
                                char.vice = v;
                            });
                    },
                    for v in types::Vice::ALL {
                        option { value: "{v}", selected: vice() == v, "{v}" }
                    }
                }
            }

            hr { class: "my-2" }
            div { class: "flex flex-col gap-2 justify-between",
            div {
                class: "flex flex-row gap-2 items-center justify-between",
                div {
                    span { "Stress" }
                    span { class: "flex flex-row gap-1 items-center",
                        for i in 0..9 {
                            CountBtn {
                                this: i,
                                total: stress(),
                                readonly: !editing(),
                                set: move |s| {
                                    character
                                        .with_mut(|char| {
                                            char.stress = s;
                                        });
                                },
                            }
                        }
                    }
                }
                div {
                    span { "Trauma" }
                    span { class: "flex flex-row gap-1 items-center",
                        for i in 0..4 {
                            CountBtn {
                                this: i,
                                total: character().trauma.count_bits(),
                                readonly: true,
                                set: |_| {}
                            }
                        }
                    }
                }
            }
            div {
            class: "flex flex-row flex-wrap gap-2 items-center justify-center",
                for trauma in types::Trauma::ALL {
                    Trauma { trauma, character }
                }
            }
            }
            hr { class: "my-2" }
        }
    }
}

#[component]
fn Trauma(
    trauma: types::Trauma,
    readonly: Option<bool>,
    character: Signal<types::Character>,
) -> Element {
    let has_trauma = use_memo(move || character().trauma.contains(trauma.into()));

    let bg_color = if has_trauma() {
        "bg-destructive hover:bg-destructive/40 text-destructive-foreground"
    } else {
        "bg-input hover:bg-input/40"
    };

    rsx! {
            button {
                class: "cursor-pointer rounded-lg px-2 py-1 {bg_color}",
                onclick: move |_| {
                    if readonly.unwrap_or(false) {
                        return;
                    }
                    character.with_mut(|char| {
                        if has_trauma() {
                            char.trauma.remove(trauma.into());
                        } else {
                            char.trauma.insert(trauma.into());
                        }
                    });
                },
               "{trauma}"
            }
    }
}

#[component]
fn CountBtn(this: u8, total: u8, readonly: Option<bool>, set: EventHandler<u8>) -> Element {
    let class = if this < total {
        "bg-gray-300"
    } else {
        "bg-gray-500"
    };

    let hover_class = if readonly.unwrap_or(false) {
        "cursor-not-allowed"
    } else {
        "hover:bg-gray-400"
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
fn Center(character: Signal<types::Character>) -> Element {
    let class = use_signal(|| character().class);

    let abilities = use_memo(move || {
        let character = character();
        types::playbook::PLAYBOOK
            .iter()
            .filter(|ability| character.abilities.contains(&ability.name.to_string()))
            .cloned()
            .collect::<Vec<_>>()
    });
    rsx! {
        div { class: "flex flex-col gap-2 w-fit p-4",
            h1 { class: "text-6xl", "{class}" }
            for ability in abilities() {
                Ability { ability: ability.clone() }
            }
        }
    }
}

#[component]
fn Right(editing: ReadOnlySignal<bool>, character: Signal<types::Character>) -> Element {
    let coin = use_memo(move || character().coin);
    let stash = use_memo(move || character().stash);
    rsx! {
        div { class: "flex flex-row gap-2 w-fit p-4 h-32 shrink ml-auto",
            div { class: "flex flex-col w-fit h-fit",
                span { "Stash " }
                span { "Coin" }
                Coin {
                    coin,
                    readonly: !editing(),
                    set: move |c| {
                        character
                            .with_mut(|char| {
                                char.coin = c;
                            });
                    },
                }
            }
            Stash {
                stash,
                readonly: !editing(),
                set: move |s| {
                    character
                        .with_mut(|c| {
                            c.stash = s;
                        });
                },
            }
        }
    }
}

#[component]
fn Ability(ability: types::playbook::Ability) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1",
            span { class: "flex flex-row gap-2 items-center justify-between",
                h2 { class: "text-2xl w-fit underline", "{ability.name}" }
                p { class: "italic", "{ability.class}" }
            }
            Description { desc: ability.description }
        }
    }
}

#[component]
fn DropdownList<T: std::fmt::Display + Clone + PartialEq + From<String> + 'static>(
    name: &'static str,
    value: String,
    set: EventHandler<T>,
    readonly: Option<bool>,
    children: Element,
) -> Element {
    let readonly = readonly.unwrap_or(false);

    rsx! {
        label { class: "grid gap-2 items-center col-span-2 grid-cols-subgrid",
            span { "{name}" }
            if readonly {
                span { class: "bg-input rounded-lg px-2 py-1", "{value}" }
            } else {
                select {
                    value: "{value}",
                    onchange: move |e| {
                        set(e.value().into());
                    },
                    {children}
                }
            }
        }
    }
}

#[component]
fn Stash(stash: ReadOnlySignal<u8>, readonly: Option<bool>, set: EventHandler<u8>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1 w-full h-fit",
            for y in 0..4 {
                div { class: "flex flex-row gap-1 w-full h-fit",
                    for x in 0..10 {
                        CountBtn {
                            this: (y * 10) + x,
                            readonly,
                            total: stash(),
                            set,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Coin(coin: ReadOnlySignal<u8>, readonly: Option<bool>, set: EventHandler<u8>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1 w-full h-max",
            for y in 0..2 {
                div { class: "flex flex-row gap-1 w-full h-max",
                    for x in 0..2 {
                        CountBtn {
                            this: (y * 2) + x,
                            total: coin(),
                            readonly,
                            set,
                        }
                    }
                }
            }
        }
    }
}
