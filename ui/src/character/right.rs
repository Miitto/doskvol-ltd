use dioxus::prelude::*;
use types::Character;

use crate::common::{CountBtn, ItemChecked};

#[component]
pub fn Right(readonly: ReadOnlySignal<bool>, character: Signal<types::Character>) -> Element {
    let coin = use_memo(move || character().coin);
    let stash = use_memo(move || character().stash);
    rsx! {
        div { class: "flex flex-col flex-auto lg:max-w-fit shrink p-4 pt-2 lg:pl-2 lg:pt-4",
            div { class: "flex flex-row gap-2 h-32 ",
                div { class: "flex flex-col w-fit h-fit",
                    span { "Stash " }
                    span { "Coin" }
                    Coin {
                        coin,
                        readonly: readonly(),
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
                    readonly: readonly(),
                    set: move |s| {
                        character
                            .with_mut(|c| {
                                c.stash = s;
                            });
                    },
                }
            }

            Xp { character, readonly}
            Items { character, readonly }
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

#[component]
fn Xp(character: Signal<Character>, readonly: ReadOnlySignal<bool>) -> Element {
    rsx! {}
}

#[component]
fn Items(character: Signal<Character>, readonly: ReadOnlySignal<bool>) -> Element {
    let load = use_memo(move || character().load);
    rsx! {
        div { class: "flex flex-row gap-2 items-center justify-between mb-4",
            ItemChecked { checked: load().is_some_and(|load| matches!(load, types::Load::Light)), readonly, onclick: move |set| character.with_mut(|c| { c.load = if set { Some(types::Load::Light) } else { None }; })}
            span {
                span { class: "font-bold", "3"}
                span { class: "italic", " light" }
            }
            ItemChecked { checked: load().is_some_and(|load| matches!(load, types::Load::Medium)), readonly, onclick: move |set| character.with_mut(|c| { c.load = if set { Some(types::Load::Medium) } else { None }; })}
            span {
                span { class: "font-bold", "5"}
                span { class: "italic", " medium" }
            }
            ItemChecked { checked: load().is_some_and(|load| matches!(load, types::Load::Heavy)), readonly, onclick: move |set| character.with_mut(|c| { c.load = if set { Some(types::Load::Heavy) } else { None }; })}
            span {
                span { class: "font-bold", "6"}
                span { class: "italic", " heavy" }
            }
        }
        Item { flag: types::Items::BLADE, boxes: 1, character, readonly, "A Blade or Two" }
        Item { flag: types::Items::THROWING_KNIVES, boxes: 1, character, readonly, "Throwing Knives" }
        div {
            class: "inline-flex justify-between gap-2 w-full",
            Item { flag: types::Items::PISTOL, boxes: 1, character, readonly, "A Pistol" }
            Item { flag: types::Items::PISTOL_2, boxes: 1, character, readonly, "A 2" sup {"nd"} " Pistol" }
        }
        Item { flag: types::Items::LARGE_WEAPON, boxes: 2, character, readonly, "A Large Weapon" }
        Item { flag: types::Items::UNUSUAL_WEAPON, boxes: 1, character, readonly, "An Unusual Weapon" }
        div { class: "inline-flex justify-between gap-2 w-full",
            Item { flag: types::Items::ARMOR, boxes: 2, character, readonly, "Armor" }
            Item { flag: types::Items::HEAVY_ARMOR, boxes: 3, character, readonly, "+ Heavy" }
        }
        Item { flag: types::Items::BURGLARY_GEAR, boxes: 1, character, readonly, "Burglary Gear" }
        Item { flag: types::Items::CLIMBING_GEAR, boxes: 2, character, readonly, "Climbing Gear" }
        Item { flag: types::Items::ARCANE_IMPLEMENTS, boxes: 1, character, readonly, "Arcane Implements" }
        Item { flag: types::Items::DOCUMENTS, boxes: 1, character, readonly, "Documents" }
        Item { flag: types::Items::SUBTERFUGE_SUPPLIES, boxes: 1, character, readonly, "Subterfuge Supplies" }
        Item { flag: types::Items::DEMO_TOOLS, boxes: 2, character, readonly, "Demolition Tools" }
        Item { flag: types::Items::TINKER_TOOLS, boxes: 1, character, readonly, "Tinkering Tools" }
        Item { flag: types::Items::LANTERN, boxes: 1, character, readonly, "Lantern" }
    }
}

#[component]
fn Item(
    flag: types::Items,
    boxes: u8,
    character: Signal<Character>,
    readonly: ReadOnlySignal<bool>,
    children: Element,
) -> Element {
    let has_item = use_memo(move || character().items.contains(flag));
    let connector_background = if has_item() {
        "bg-neutral-300"
    } else {
        "bg-neutral-500"
    };
    let hover = if readonly() {
        "cursor-not-allowed"
    } else {
        "group-hover:bg-neutral-400"
    };
    rsx! {
        div { class: "flex flex-row gap-2 items-center",
        div { class: "group flex flex-row items-center",
            for i in 0..boxes {
                    if i != 0 {
                        div { class: "w-2 h-1 {connector_background} {hover}" }
                    }
                    ItemChecked { checked: has_item(), readonly, onclick: move |_| character.with_mut(|c| { c.items.toggle(flag); })}
                }
            }
            span { {children} }
        }
    }
}
