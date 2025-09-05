use dioxus::prelude::*;
use types::{Class, Description as DescriptionT};

use crate::{
    common::ItemChecked,
    elements::{Description, Dialog},
};

#[component]
pub fn Center(character: Signal<types::Character>, readonly: ReadOnlySignal<bool>) -> Element {
    let class = use_signal(|| character().class);
    let mut open = use_signal(|| false);

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
            div { class: "flex flex-col gap-4",
                for ability in abilities() {
                    Ability { ability: ability.clone() }
                }
                if !readonly() {
                    div { class: "inline-flex justify-end pt-2",
                        button {
                            class: "bg-primary text-primary-foreground w-fit rounded-lg p-2 cursor-pointer",
                            onclick: move |_| open.set(true),
                            "Modify"
                        }
                    }
                    AbilityDialog { open, character }
                }
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
        div { class: "flex flex-col gap-1 w-full",
            span { class: "flex flex-row gap-2 items-center justify-between",
                h2 { class: "text-2xl w-fit underline", "{ability.name}" }
                p { class: "italic", "{ability.class}" }
            }
            Description { desc: ability.description }
        }
    }
}

#[component]
fn AbilityDialog(open: Signal<bool>, character: Signal<types::Character>) -> Element {
    let abilities = use_memo(move || {
        let mut a = types::playbook::PLAYBOOK.to_vec();
        a.sort_by(|a, b| {
            if a.class != character().class && b.class != character().class {
                std::cmp::Ordering::Equal
            } else if a.class == character().class && b.class != character().class {
                std::cmp::Ordering::Less
            } else if a.class != character().class && b.class == character().class {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        a
    });

    rsx! {
        Dialog { open, close_on_click: true,
            div { class: "flex flex-col gap-4 w-full h-full",
                h2 { class: "text-3xl", "Abilities" }
                hr {}
                div { class: "flex flex-col gap-4 max-h-full overflow-y-auto",
                    for ability in abilities() {
                        AbilityButton { ability, character }
                    }
                }
                div { class: "inline-flex justify-end",
                    button {
                        class: "bg-primary text-primary-foreground w-fit rounded-lg p-2 cursor-pointer",
                        onclick: move |_| open.set(false),
                        "Close"
                    }
                }
            }
        }
    }
}

#[component]
fn AbilityButton(
    ability: types::playbook::Ability,
    character: Signal<types::Character>,
) -> Element {
    let has_ability = use_memo(move || character().abilities.contains(&ability.name.to_string()));
    let name = ability.name.to_string();

    let color = if has_ability() {
        "bg-primary hover:bg-primary/80 text-primary-foreground"
    } else {
        "bg-background hover:bg-input text-foreground"
    };

    rsx! {
        button {
            class: "{color} cursor-pointer p-2",
            onclick: move |e| {
                e.stop_propagation();
                let name = name.clone();
                let name_a = name.clone();
                let id = character().id;
                let has = has_ability();
                spawn(async move {
                    if has {
                        let res = api::character::remove_ability(id, name_a).await;
                        #[cfg(debug_assertions)]
                        {
                            if let Err(e) = &res {
                                tracing::error!("Failed to remove ability: {e}");
                            }
                        }
                    } else {
                        let res = api::character::add_ability(id, name_a).await;
                        #[cfg(debug_assertions)]
                        {
                            if let Err(e) = &res {
                                tracing::error!("Failed to add ability: {e}");
                            }
                        }
                    }
                });
                character
                    .with_mut(move |char| {
                        if has {
                            char.abilities.retain(|a| *a != name);
                        } else {
                            char.abilities.push(name);
                        }
                    });
            },
            Ability { ability }
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
                            spawn(async move {
                                let res = api::character::add_contact(
                                        character().id,
                                        contact.to_string(),
                                        true,
                                    )
                                    .await;
                                #[cfg(debug_assertions)]
                                {
                                    if let Err(e) = &res {
                                        tracing::error!("Failed to add contact: {e}");
                                    }
                                }
                            });
                        },
                        remove: move || {
                            character
                                .with_mut(|char| {
                                    char.contacts.friends.retain(|c| c != contact);
                                });
                            spawn(async move {
                                let res = api::character::remove_contact(
                                        character().id,
                                        contact.to_string(),
                                        true,
                                    )
                                    .await;
                                #[cfg(debug_assertions)]
                                {
                                    if let Err(e) = &res {
                                        tracing::error!("Failed to add contact: {e}");
                                    }
                                }
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
                            spawn(async move {
                                let res = api::character::add_contact(
                                        character().id,
                                        contact.to_string(),
                                        false,
                                    )
                                    .await;
                                #[cfg(debug_assertions)]
                                {
                                    if let Err(e) = &res {
                                        tracing::error!("Failed to add contact: {e}");
                                    }
                                }
                            });
                        },
                        remove: move || {
                            character
                                .with_mut(|char| {
                                    char.contacts.rivals.retain(|c| c != contact);
                                });
                            spawn(async move {
                                let res = api::character::remove_contact(
                                        character().id,
                                        contact.to_string(),
                                        false,
                                    )
                                    .await;
                                #[cfg(debug_assertions)]
                                {
                                    if let Err(e) = &res {
                                        tracing::error!("Failed to add contact: {e}");
                                    }
                                }
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

    let cursor = if readonly {
        "cursor-not-allowed"
    } else {
        "cursor-pointer"
    };

    rsx! {
        button { class: "{fill} {hover} {cursor}", onclick: move |_| onclick.call(()),
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
                        checked: character().class_items.contains(&item.to_string()),
                        onclick: move |has| {
                            spawn(async move {
                                let res = if has {
                                    api::character::add_class_item(character().id, item.to_string()).await
                                } else {
                                    api::character::remove_class_item(character().id, item.to_string()).await
                                };
                                #[cfg(debug_assertions)]
                                {
                                    if let Err(e) = &res {
                                        tracing::error!("Failed to modify class item: {e}");
                                    }
                                }
                            });
                            character
                                .with_mut(|char| {
                                    if has {
                                        char.class_items.push(item.to_string());
                                    } else {
                                        char.class_items.retain(|i| i != item);
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
