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

            Xp { character, readonly }
            br {}
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
                            this: (y * 10) + x + 1,
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
                            this: (y * 2) + x + 1,
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
    rsx! {
        XpLine {
            name: "Playbook",
            readonly,
            max: 8,
            current: character().xp.playbook,
            set: move |count| {
                character
                    .with_mut(|c| {
                        c.xp.playbook = count;
                    })
            },
        }

        div { class: "flex flex-row lg:flex-col gap-4 justify-between flex-wrap lg:flex-no-wrap lg:justify-start pt-4",
            div { class: "flex flex-col",

                XpLine {
                    name: "Insight",
                    readonly,
                    max: 6,
                    current: character().xp.insight,
                    set: move |count| {
                        character
                            .with_mut(|c| {
                                c.xp.insight = count;
                            })
                    },
                }
                DotBlock {
                    readonly,
                    params: DotBlockParamList(
                        DotBlockParams {
                            name: "Hunt",
                            current: character().dots.hunt,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.hunt = count)
                            }),
                        },
                        DotBlockParams {
                            name: "Study",
                            current: character().dots.study,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.study = count)
                            }),
                        },
                        DotBlockParams {
                            name: "Survey",
                            current: character().dots.survey,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.survey = count)
                            }),
                        },
                        DotBlockParams {
                            name: "Tinker",
                            current: character().dots.tinker,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.tinker = count)
                            }),
                        },
                    ),
                }
            }
            div { class: "flex flex-col",
                XpLine {
                    name: "Prowess",
                    readonly,
                    max: 6,
                    current: character().xp.prowess,
                    set: move |count| {
                        character
                            .with_mut(|c| {
                                c.xp.prowess = count;
                            })
                    },
                }


                DotBlock {
                    readonly,
                    params: DotBlockParamList(
                        DotBlockParams {
                            name: "Finesse",
                            current: character().dots.finesse,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.finesse = count)
                            }),
                        },
                        DotBlockParams {
                            name: "Prowl",
                            current: character().dots.prowl,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.prowl = count)
                            }),
                        },
                        DotBlockParams {
                            name: "Skirmish",
                            current: character().dots.skirmish,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.skirmish = count)
                            }),
                        },
                        DotBlockParams {
                            name: "Wreck",
                            current: character().dots.wreck,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.wreck = count)
                            }),
                        },
                    ),
                }
            }

            div { class: "flex flex-col",
                XpLine {
                    name: "Resolve",
                    readonly,
                    max: 6,
                    current: character().xp.resolve,
                    set: move |count| {
                        character
                            .with_mut(|c| {
                                c.xp.resolve = count;
                            })
                    },
                }
                DotBlock {
                    readonly,
                    params: DotBlockParamList(
                        DotBlockParams {
                            name: "Attune",
                            current: character().dots.attune,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.attune = count)
                            }),
                        },
                        DotBlockParams {
                            name: "Command",
                            current: character().dots.command,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.command = count)
                            }),
                        },
                        DotBlockParams {
                            name: "Consort",
                            current: character().dots.consort,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.consort = count)
                            }),
                        },
                        DotBlockParams {
                            name: "Sway",
                            current: character().dots.sway,
                            set: EventHandler::new(move |count| {
                                character.with_mut(|c| c.dots.sway = count)
                            }),
                        },
                    ),
                }
            }
        }
    }
}

#[component]
fn XpLine(
    name: &'static str,
    readonly: ReadOnlySignal<bool>,
    max: u8,
    current: u8,
    set: EventHandler<u8>,
) -> Element {
    rsx! {
        div { class: "flex flex-row gap-2 lg:justify-between",
            span { class: "font-bold", "{name}" }
            div { class: "flex flex-row gap-1",
                for i in 1..=max {
                    CountBtn {
                        this: i,
                        total: current,
                        set: move |count| {
                            set.call(count);
                        },
                        readonly: readonly(),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct DotBlockParams {
    name: &'static str,
    current: u8,
    set: EventHandler<u8>,
}

#[derive(Debug, Clone, PartialEq)]
struct DotBlockParamList(
    DotBlockParams,
    DotBlockParams,
    DotBlockParams,
    DotBlockParams,
);

#[component]
fn DotBlock(readonly: ReadOnlySignal<bool>, params: DotBlockParamList) -> Element {
    rsx! {

        div { class: "grid grid-cols-[auto_auto_auto_auto_0.25rem_1fr] grid-rows-4 pt-2 grid-flow-col",
            div { class: "grid grid-cols-subgrid grid-rows-subgrid row-span-4 pr-1 border-r border-border items-center",
                CountBtn {
                    class: "rounded-full",
                    this: 1,
                    total: params.0.current,
                    readonly: readonly(),
                    set: params.0.set,
                }
                CountBtn {
                    class: "rounded-full",
                    this: 1,
                    total: params.1.current,
                    readonly: readonly(),
                    set: params.1.set,
                }
                CountBtn {
                    class: "rounded-full",
                    this: 1,
                    total: params.2.current,
                    readonly: readonly(),
                    set: params.2.set,
                }
                CountBtn {
                    class: "rounded-full",
                    this: 1,
                    total: params.3.current,
                    readonly: readonly(),
                    set: params.3.set,
                }
            }
            div { class: "grid grid-cols-subgrid grid-rows-subgrid row-span-4 col-span-5 pl-1 gap-1 items-center",
                DotLine { readonly, params: params.0 }
                DotLine { readonly, params: params.1 }
                DotLine { readonly, params: params.2 }
                DotLine { readonly, params: params.3 }
            }
        }
    }
}

#[component]
fn DotLine(readonly: ReadOnlySignal<bool>, params: DotBlockParams) -> Element {
    rsx! {
        for i in 2..=4 {
            CountBtn {
                class: "rounded-full",
                this: i,
                total: params.current,
                set: params.set,
            }
        }
        div {}
        span { class: "font-bold", "{params.name}" }
    }
}

#[component]
fn Items(character: Signal<Character>, readonly: ReadOnlySignal<bool>) -> Element {
    let load = use_memo(move || character().load);
    rsx! {
        div { class: "flex flex-row gap-2 items-center lg:justify-between mb-4",
            ItemChecked {
                checked: load().is_some_and(|load| matches!(load, types::Load::Light)),
                readonly,
                onclick: move |set| {
                    character
                        .with_mut(|c| {
                            c.load = if set { Some(types::Load::Light) } else { None };
                        })
                },
            }
            span {
                span { class: "font-bold", "3" }
                span { class: "italic", " light" }
            }
            ItemChecked {
                checked: load().is_some_and(|load| matches!(load, types::Load::Medium)),
                readonly,
                onclick: move |set| {
                    character
                        .with_mut(|c| {
                            c.load = if set { Some(types::Load::Medium) } else { None };
                        })
                },
            }
            span {
                span { class: "font-bold", "5" }
                span { class: "italic", " medium" }
            }
            ItemChecked {
                checked: load().is_some_and(|load| matches!(load, types::Load::Heavy)),
                readonly,
                onclick: move |set| {
                    character
                        .with_mut(|c| {
                            c.load = if set { Some(types::Load::Heavy) } else { None };
                        })
                },
            }
            span {
                span { class: "font-bold", "6" }
                span { class: "italic", " heavy" }
            }
        }
        Item {
            flag: types::Items::BLADE,
            boxes: 1,
            character,
            readonly,
            "A Blade or Two"
        }
        Item {
            flag: types::Items::THROWING_KNIVES,
            boxes: 1,
            character,
            readonly,
            "Throwing Knives"
        }
        div { class: "inline-flex lg:justify-between gap-4 w-full",
            Item {
                flag: types::Items::PISTOL,
                boxes: 1,
                character,
                readonly,
                "A Pistol"
            }
            Item {
                flag: types::Items::PISTOL_2,
                boxes: 1,
                character,
                readonly,
                "A 2"
                sup { "nd" }
                " Pistol"
            }
        }
        Item {
            flag: types::Items::LARGE_WEAPON,
            boxes: 2,
            character,
            readonly,
            "A Large Weapon"
        }
        Item {
            flag: types::Items::UNUSUAL_WEAPON,
            boxes: 1,
            character,
            readonly,
            "An Unusual Weapon"
        }
        div { class: "inline-flex lg:justify-between gap-4 w-full",
            Item {
                flag: types::Items::ARMOR,
                boxes: 2,
                character,
                readonly,
                "Armor"
            }
            Item {
                flag: types::Items::HEAVY_ARMOR,
                boxes: 3,
                character,
                readonly,
                "+ Heavy"
            }
        }
        Item {
            flag: types::Items::BURGLARY_GEAR,
            boxes: 1,
            character,
            readonly,
            "Burglary Gear"
        }
        Item {
            flag: types::Items::CLIMBING_GEAR,
            boxes: 2,
            character,
            readonly,
            "Climbing Gear"
        }
        Item {
            flag: types::Items::ARCANE_IMPLEMENTS,
            boxes: 1,
            character,
            readonly,
            "Arcane Implements"
        }
        Item {
            flag: types::Items::DOCUMENTS,
            boxes: 1,
            character,
            readonly,
            "Documents"
        }
        Item {
            flag: types::Items::SUBTERFUGE_SUPPLIES,
            boxes: 1,
            character,
            readonly,
            "Subterfuge Supplies"
        }
        Item {
            flag: types::Items::DEMO_TOOLS,
            boxes: 2,
            character,
            readonly,
            "Demolition Tools"
        }
        Item {
            flag: types::Items::TINKER_TOOLS,
            boxes: 1,
            character,
            readonly,
            "Tinkering Tools"
        }
        Item {
            flag: types::Items::LANTERN,
            boxes: 1,
            character,
            readonly,
            "Lantern"
        }
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
        "bg-primary"
    } else {
        "bg-primary/50"
    };
    let hover = if readonly() {
        "cursor-not-allowed"
    } else {
        "group-hover:brightness-80"
    };
    rsx! {
        div { class: "flex flex-row gap-2 items-center",
            div { class: "group flex flex-row items-center",
                for i in 0..boxes {
                    if i != 0 {
                        div { class: "w-1 h-px {connector_background} {hover}" }
                    }
                    ItemChecked {
                        checked: has_item(),
                        readonly,
                        onclick: move |_| {
                            character
                                .with_mut(|c| {
                                    c.items.toggle(flag);
                                })
                        },
                    }
                }
            }
            span { {children} }
        }
    }
}
