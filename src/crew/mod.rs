use dioxus::prelude::*;

mod create;
pub use create::CreateCrew;

use crate::{character::CreateCharacter, elements::Dialog};

#[component]
pub fn Crew<R: 'static + Clone + PartialEq + Routable>(
    crew: types::Crew,
    to_character_page: Callback<types::CrewId, R>,
) -> Element {
    let id = crew.id;
    let mut crew_characters =
        use_server_future(
            move || async move { api::crew::get_crew_characters(id).await.unwrap() },
        )?;

    let mut open_create_character = use_signal(|| false);

    let mut show_invites = use_signal(|| false);

    let auth: crate::Auth = use_context();

    let is_dm = use_memo(move || auth.username().is_some_and(|u| u == crew.dm_id));

    rsx! {
        div { class: "flex flex-col gap-4 p-4",

            if is_dm() {
                div { class: "flex flex-row gap-4 items-center",
                    button {
                        class: "p-2 bg-muted text-muted-foreground rounded-lg cursor-pointer",
                        onclick: move |_| {
                            show_invites.set(true);
                        },
                        "Invites"
                    }
                }
            }

            if let Some(crew_characters) = crew_characters() {
                div { class: "flex flex-col grow gap-2",
                    for character in crew_characters {
                        Link {
                            class: "hover:bg-input hover:text-input-foreground p-2 rounded-lg",
                            to: to_character_page.call(character.id),
                            div { class: "flex flex-row justify-between items-center gap-2",
                                div {
                                    h2 { class: "text-xl", "{character.name}" }
                                    p { class: "italic", "{character.class}" }
                                }

                                p { class: "italic", "{character.player_name}" }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-row justify-end",
                button {
                    class: "p-2 bg-primary text-primary-foreground rounded-lg cursor-pointer",
                    onclick: move |_| {
                        open_create_character.set(true);
                    },
                    "Create Character"
                }
            }
        }

        CreateCharacter {
            crew_id: crew.id,
            on_create: move |new_character| async move {
                let res = api::character::create(new_character).await;
                if let Err(err) = res {
                    tracing::error!("Failed to create crew: {:?}", err);
                } else {
                    crew_characters.restart();
                }
            },

            open: open_create_character,
        }

        InvitesDialog { open: show_invites, crew_id: crew.id }
    }
}

#[component]
fn InvitesDialog(open: Signal<bool>, crew_id: ReadOnlySignal<types::CrewId>) -> Element {
    let mut invites = use_server_future(move || async move {
        api::crew::get_invites(crew_id()).await.unwrap_or_default()
    })?;

    let mut new_invite_max = use_signal(|| 1);

    rsx! {
        Dialog { open, close_on_click: true,
            div { class: "flex flex-col gap-4",
                for invite in invites().unwrap_or_default() {
                    div { class: "flex flex-row justify-between items-center",
                        div {
                            div { "Code: {invite.code}" }
                            div { "Uses: {invite.used} / {invite.max_uses}" }
                        }
                        button {
                            class: "p-2 bg-primary text-primary-foreground rounded-lg cursor-pointer",
                            onclick: move |_| {
                                let code = invite.code.clone();
                                async move {
                                    let _ = api::crew::delete_invite(code).await;
                                    invites.restart();
                                }
                            },
                            "Revoke"
                        }
                    }
                }

                div { class: "flex flex-row justify-between items-center gap-8 mt-4",
                    button {
                        class: "p-2 bg-muted text-muted-foreground rounded-lg cursor-pointer",
                        onclick: move |_| {
                            open.set(false);
                        },
                        "Close"
                    }
                    div { class: "flex flex-row items-center gap-4",
                        div { class: "flex flex-row items-center gap-2",
                            label { "Max Uses:" }
                            input {
                                class: "p-1 rounded-lg border border-border w-16",
                                r#type: "number",
                                value: "{new_invite_max}",
                                oninput: move |e| {
                                    if let Ok(value) = e.value().parse::<i32>() {
                                        new_invite_max.set(value);
                                    }
                                },
                                min: "1",
                            }
                        }
                        button {
                            class: "p-2 bg-primary text-primary-foreground rounded-lg cursor-pointer",
                            onclick: move |_| async move {
                                let max_uses = new_invite_max();
                                if max_uses < 1 {
                                    return;
                                }
                                let res = api::crew::create_invite(crew_id(), max_uses).await;
                                if let Err(err) = res {
                                    tracing::error!("Failed to create invite: {:?}", err);
                                } else {
                                    invites.restart();
                                }
                            },
                            "Create Invite"
                        }
                    }
                }
            }
        }
    }
}
