use dioxus::prelude::*;

mod register;
pub use register::*;

#[component]
pub fn Login(register: NavigationTarget, on_login: EventHandler) -> Element {
    let mut username = use_signal(String::default);
    let mut totp = use_signal(String::default);

    let mut error = use_signal(|| None as Option<String>);

    let mut auth: crate::Auth = use_context();

    rsx! {
        div { class: "flex justify-center pt-30 w-full h-full",
        form { class: "flex flex-col gap-4 container h-fit p-4 border border-border rounded",
            onsubmit: move |e| async move {
                e.prevent_default();

                let user = api::login(username(), totp()).await;

                if let Err(err) = user {
                    #[cfg(debug_assertions)]
                    {
                        tracing::error!("Login error: {:?}", err);
                    }
                    error.set(Some("Invalid username or authenticator code".into()))
                } else {
                    auth.refresh();
                    on_login.call(());
                }
            },

            input {
                class: "bg-input p-2 rounded",
                type: "text",
                placeholder: "Username",
                value: "{username}",
                onchange: move |e| username.set(e.value()),
            }

            input { type: "text", class: "bg-input p-2 rounded", placeholder: "Authenticator code", value: "{totp}", onchange: move |e| totp.set(e.value()), }

            if let Some(error) = error() {
                p { class: "text-destructive", "{error}" }
            }

            div { class: "flex justify-between items-center",
                Link {
                    to: register,
                    "Register"
                }
                button { class: "bg-primary text-primary-foreground rounded px-4 py-2 hover:bg-primary/90 transition", "Log In" }
            }
        }
        }
    }
}
