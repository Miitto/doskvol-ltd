use dioxus::prelude::*;
use types::{ArmorFlags, BitCount};

use crate::common::CountBtn;
use crate::elements::{Description, DescriptionEdit};

#[component]
pub fn Character(character: types::Character, readonly: Option<bool>) -> Element {
    let character = use_signal(|| character);

    let readonly = use_memo(move || !readonly.unwrap_or(true));

    rsx! {
        div { class: "flex flex-col lg:flex-row h-full",
            Left { readonly, character }
            div { class: "bg-border min-h-px h-px w-full lg:w-px lg:h-full" }
            Center { character }
            div { class: "bg-border min-h-px h-px w-full lg:w-px lg:h-full" }
            Right { character, readonly }
        }
    }
}

#[component]
fn Left(readonly: ReadOnlySignal<bool>, mut character: Signal<types::Character>) -> Element {
    let name = use_signal(|| character().name);
    let look = use_signal(|| character().look);
    let heritage = use_memo(move || character().heritage);
    let background = use_memo(move || character().background);
    let vice = use_memo(move || character().vice);
    let stress = use_memo(move || character().stress);

    rsx! {
        div { class: "flex flex-col gap-2 flex-auto p-4 pb-2 lg:pr-2 lg:pb-4",
            h2 { class: "block text-4xl w-max", "{name}" }
            if readonly() {
                input { class: "p-1", value: "{look}" }
            } else {
                Description { desc: look() }
            }

            hr { class: "my-2" }

            div { class: "grid grid-cols-2 gap-2",
                DropdownList {
                    name: "Herritage",
                    value: "{heritage}",
                    readonly: !readonly(),
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
                    readonly: !readonly(),
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
                    readonly: !readonly(),
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
                div { class: "flex flex-row gap-2 items-center justify-between",
                    div {
                        span { "Stress" }
                        span { class: "flex flex-row gap-1 items-center",
                            for i in 0..9 {
                                CountBtn {
                                    this: i,
                                    total: stress(),
                                    readonly: !readonly(),
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
                                    set: |_| {},
                                }
                            }
                        }
                    }
                }
                div { class: "flex flex-row flex-wrap gap-2 items-center justify-center",
                    for trauma in types::Trauma::ALL {
                        Trauma { trauma, character }
                    }
                }
            }
            hr { class: "my-2" }
            Harm { character }

            div {
                class: "flex flex-row flex-wrap justify-between gap-4 items-center",
                div { class: "flex flex-row gap-2 items-center",
                    span { "Healing" }
                    div { class: "flex flex-row gap-1 items-center",
                        for i in 0..4 {
                            CountBtn {
                                this: i,
                                total: character().healing,
                                set: move |h| {
                                    character
                                        .with_mut(|char| {
                                            char.healing = h;
                                        });
                                },
                            }
                        }
                    }
                }
                div { class: "flex flex-row gap-4 items-center",
                    div { class: "flex flex-row gap-2 items-center",
                        span { "Armour" }
                        CountBtn {
                            this: if character().armor.contains(ArmorFlags::ARMOR) { 0 } else { 1 },
                            total: 1,
                            set: move |_| {
                                character.with_mut(|char| { char.armor.toggle(ArmorFlags::ARMOR) });
                            },
                        }
                    }
                    div { class: "flex flex-row gap-2 items-center",
                        span { "Heavy" }
                        CountBtn {
                            this: if character().armor.contains(ArmorFlags::HEAVY) { 0 } else { 1 },
                            total: 1,
                            set: move |_| {
                                character.with_mut(|char| { char.armor.toggle(ArmorFlags::HEAVY) });
                            },
                        }
                    }
                    div { class: "flex flex-row gap-2 items-center",
                        span { "Special" }
                        CountBtn {
                            this: if character().armor.contains(ArmorFlags::SPECIAL) { 0 } else { 1 },
                            total: 1,
                            set: move |_| {
                                character.with_mut(|char| { char.armor.toggle(ArmorFlags::SPECIAL) });
                            },
                        }
                    }
                }
            }
            hr { class: "my-2" }

            div {
                class: "flex flex-col gap-2",
                span { class: "text-lg underline", "Notes" }
            DescriptionEdit {
                desc: character().notes,
                readonly: !readonly(),
                on_change: move |desc| {
                    character.with_mut(|char| {
                        char.notes = desc;
                    });
                },
            }
            }
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
        div { class: "flex flex-col gap-2 flex-auto p-4 pb-2 pt-2 lg:pr-2 lg:pb-4 lg:pt-4 lg:pl-2",
            h1 { class: "text-6xl", "{class}" }
            for ability in abilities() {
                Ability { ability: ability.clone() }
            }
        }
    }
}

#[component]
fn Right(readonly: ReadOnlySignal<bool>, character: Signal<types::Character>) -> Element {
    let coin = use_memo(move || character().coin);
    let stash = use_memo(move || character().stash);
    rsx! {
        div { class: "flex flex-row gap-2 flex-auto lg:max-w-fit h-32 shrink p-4 pt-2 lg:pl-2 lg:pt-4",
            div { class: "flex flex-col w-fit h-fit",
                span { "Stash " }
                span { "Coin" }
                Coin {
                    coin,
                    readonly: !readonly(),
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
                readonly: !readonly(),
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
                character
                    .with_mut(|char| {
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
fn Harm(character: Signal<types::Character>) -> Element {
    let harm = use_memo(move || character().harm);

    rsx! {
        div { class: "grid grid-cols-[auto_1fr_auto]",
            HarmLine { num: 3, state: "Need Help",
                input {
                    class: "w-full h-full p-1 outline-hidden focus:outline-1 focus:outline-foreground focus:outline-solid focus:-outline-offset-1",
                    value: harm().2,
                }
            }
            HarmLine { num: 2, state: "-1D",
                input {
                    class: "w-full h-full p-1 outline-hidden focus:outline-1 focus:outline-foreground focus:outline-solid focus:-outline-offset-1",
                    value: "{harm().1[0]}",
                }
                div { class: "bg-border w-px h-full" }
                input {
                    class: "w-full h-full p-1 outline-hidden focus:outline-1 focus:outline-foreground focus:outline-solid focus:-outline-offset-1",
                    value: "{harm().1[1]}",
                }
            }
            HarmLine { num: 1, state: "Less Effect",
                input {
                    class: "w-full h-full p-1 outline-hidden focus:outline-1 focus:outline-foreground focus:outline-solid focus:-outline-offset-1",
                    value: "{harm().0[0]}",
                }
                div { class: "bg-border w-px h-full" }
                input {
                    class: "w-full h-full p-1 outline-hidden focus:outline-1 focus:outline-foreground focus:outline-solid focus:-outline-offset-1",
                    value: "{harm().0[1]}",
                }
            }
        }
    }
}

#[component]
fn HarmLine(num: u8, state: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "grid grid-cols-subgrid col-span-3 border-b border-border first:border-t",
            span { class: "text-sm p-1 bg-neutral-800 h-full", "{num}" }
            div { class: "flex flex-row grow w-full", {children} }
            span { class: "text-sm p-1 grow text-wrap h-full bg-neutral-800 text-end", "{state}" }
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
