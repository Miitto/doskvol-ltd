use dioxus::prelude::*;
use types::{Class, Description as DescriptionT};

use crate::{common::ItemChecked, elements::Description};

#[component]
pub fn Center(character: Signal<types::Character>, readonly: ReadOnlySignal<bool>) -> Element {
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
            hr { class: "my-2" }
            div { class: "flex flex-row flex-wrap gap-4 justify-between",
                SlyFriends { character, readonly }
                ClassItems { character, readonly }
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
fn SlyFriends(character: Signal<types::Character>, readonly: ReadOnlySignal<bool>) -> Element {
    let sly_friends = use_memo(move || character().contacts.clone());
    let friends = use_memo(move || sly_friends().friends);
    let rivals = use_memo(move || sly_friends().rivals);

    let contacts = use_memo(move || {
        use types::contacts::CONTACTS;
        let contacts: &[&'static str] = match character().class {
            Class::Cutter => &CONTACTS.cutter,
            Class::Hound => &CONTACTS.hound,
            Class::Leech => &CONTACTS.leech,
            Class::Lurk => &CONTACTS.lurk,
            Class::Slide => &CONTACTS.slide,
            Class::Spider => &CONTACTS.spider,
            Class::Whisper => &CONTACTS.whisper,
        };
        contacts
    });

    rsx! {
        div {
            h3 { class: "text-lg underline", "Sly Friends" }
            for contact in contacts() {
                div { class: "flex flex-row gap-2 items-center",
                    ContactTriangle {
                        readonly,
                        flip: false,
                        fill: friends().contains(&contact.to_string()),
                        add: move || {
                            character
                                .with_mut(|char| {
                                    char.contacts.friends.push(contact.to_string());
                                    char.contacts.rivals.retain(|c| c != contact);
                                });
                        },
                        remove: move || {
                            character
                                .with_mut(|char| {
                                    char.contacts.friends.retain(|c| c != contact);
                                });
                        },
                    }
                    ContactTriangle {
                        readonly,
                        flip: true,
                        fill: rivals().contains(&contact.to_string()),
                        add: move || {
                            character
                                .with_mut(|char| {
                                    char.contacts.rivals.push(contact.to_string());
                                    char.contacts.friends.retain(|c| c != contact);
                                });
                        },
                        remove: move || {
                            character
                                .with_mut(|char| {
                                    char.contacts.rivals.retain(|c| c != contact);
                                });
                        },
                    }
                    span { "{contact}" }
                }
            }
        }
    }
}

#[component]
fn ContactTriangle(
    flip: bool,
    fill: bool,
    add: EventHandler,
    remove: EventHandler,
    readonly: ReadOnlySignal<bool>,
) -> Element {
    rsx! {
        Triangle {
            readonly: readonly(),
            flip,
            fill,
            onclick: move |_| {
                if readonly() {
                    return;
                }
                if fill {
                    remove.call(());
                } else {
                    add.call(());
                }
            },
        }
    }
}

#[component]
fn Triangle(flip: bool, fill: bool, onclick: EventHandler, readonly: Option<bool>) -> Element {
    let readonly = readonly.unwrap_or(true);
    let stroke = if fill {
        "none"
    } else {
        "var(--color-foreground)"
    };
    let fill = if fill {
        "text-foreground"
    } else {
        "text-transparent"
    };
    let flip = if flip { "rotate-180" } else { "" };
    let hover = if readonly {
        ""
    } else {
        "hover:text-foreground/50"
    };
    rsx! {
        button { class: "{fill} {hover}", onclick: move |_| onclick.call(()),
            svg { class: "w-4 h-4 {flip}", view_box: "0 0 100 100",
                polygon {
                    points: "50,0 0,100 100,100",
                    fill: "currentColor",
                    stroke,
                    stroke_width: "10",
                }
            }
        }
    }
}

#[component]
fn ClassItems(character: Signal<types::Character>, readonly: ReadOnlySignal<bool>) -> Element {
    let items = use_memo(move || {
        use types::items::CLASS_ITEMS;
        let items: &[DescriptionT<&'static str>] = match character().class {
            Class::Cutter => &CLASS_ITEMS.cutter,
            Class::Hound => &CLASS_ITEMS.hound,
            Class::Leech => &CLASS_ITEMS.leech,
            Class::Lurk => &CLASS_ITEMS.lurk,
            Class::Slide => &CLASS_ITEMS.slide,
            Class::Spider => &CLASS_ITEMS.spider,
            Class::Whisper => &CLASS_ITEMS.whisper,
        };
        items
    });

    rsx! {
        div {
            h3 { class: "text-lg underline", "Class Items" }
            for item in items() {
                div { class: "flex flex-row gap-2 items-center",
                    ItemChecked {
                        readonly,
                        checked: character().items.contains(&item.to_string()),
                        onclick: move |has| {
                            character
                                .with_mut(|char| {
                                    if has {
                                        char.items.push(item.to_string());
                                    } else {
                                        char.items.retain(|i| i != item);
                                    }
                                });
                        },
                    }
                    Description { desc: item.clone() }
                }
            }
        }
    }
}
