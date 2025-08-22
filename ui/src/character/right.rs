use dioxus::prelude::*;

use crate::common::CountBtn;

#[component]
pub fn Right(readonly: ReadOnlySignal<bool>, character: Signal<types::Character>) -> Element {
    let coin = use_memo(move || character().coin);
    let stash = use_memo(move || character().stash);
    rsx! {
        div { class: "flex flex-row gap-2 flex-auto lg:max-w-fit h-32 shrink p-4 pt-2 lg:pl-2 lg:pt-4",
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
