use dioxus::prelude::*;

#[component]
pub fn Register(login: NavigationTarget, on_register: EventHandler) -> Element {
    let mut username = use_signal(String::default);
    let mut validated = use_signal(|| false);

    let mut name_error = use_signal(|| None as Option<String>);

    let mut other_error = use_signal(|| None as Option<String>);

    rsx! {
        div { class: "flex justify-center pt-30 w-full h-full",
            form { class: "flex flex-col gap-4 container h-fit p-4 border border-border rounded",
                onsubmit: move |e| async move {
                    e.prevent_default();

                    if let Ok(e) = api::auth::check_username(username()).await {
                        name_error.set(e);
                        if name_error().is_none() {
                            validated.set(true);
                        } else {
                            validated.set(false);
                        }
                    } else {
                        other_error.set(Some("Failed to check username".into()));
                    }
                },
                input {
                    class: "bg-input p-2 rounded",
                    value: "{username}",
                    placeholder: "Username",
                    oninput: move |e| {username.set(e.value()); validated.set(false);},
                }

                if let Some(error) = name_error() {
                    p { class: "text-destructive", "{error}" }
                }

                if let Some(error) = other_error() {
                    p { class: "text-destructive", "{error}" }
                }

                if !validated() {
                    button { class: "bg-primary text-primary-foreground rounded px-4 py-2 hover:bg-primary/90 transition", "Check" }
                } else {
                }
                TotpSetup { username, show: validated, on_register }
            }
        }
    }
}

#[component]
fn TotpSetup(
    username: Signal<String>,
    show: ReadOnlySignal<bool>,
    on_register: EventHandler,
) -> Element {
    let totp = use_memo(move || api::auth::generate_totp_secret(username()));

    let image_data = use_memo(move || {
        if let Ok(totp) = &*totp.read() {
            totp.get_qr_base64()
        } else {
            Err("Failed to generate QR code".into())
        }
    });

    let secret = use_memo(move || {
        if let Ok(totp) = &*totp.read() {
            totp.get_secret_base32()
        } else {
            "Failed to get secret".into()
        }
    });

    let mut code = use_signal(String::default);

    let mut error = use_signal(|| None as Option<String>);

    let mut auth: Signal<crate::Auth> = use_context();

    let submit = move || async move {
        let secret = if let Ok(totp) = &*totp.read() {
            #[cfg(not(debug_assertions))]
            {
                if totp.check_current(&code()).unwrap_or(false) {
                    error.set(None);
                } else {
                    error.set(Some("Invalid authenticator code".into()));
                    return;
                }
            }
            totp.get_secret_base32()
        } else {
            return;
        };

        let user = api::auth::register(username(), secret).await;

        if let Ok(user) = user {
            auth.set(crate::Auth::Authenticated {
                username: user.username,
            });
            on_register.call(());
        } else {
            error.set(Some("Failed to register user".into()));
        }
    };

    rsx! {
        if show() {
            div { class: "flex flex-col gap-4 items-center",
                if let Ok(image_data) = image_data() {
                    div {
                        img {
                            src: "data:image/png;base64,{image_data}", alt: "QR Code"
                        }
                    }
                } else {
                    p {"Failed to generate QR code"}
                }

                p {
                    "Secret: "
                    span {
                        "{secret}"
                    }
                }

                form {
                    onsubmit: move |e| async move {
                        e.prevent_default();
                        e.stop_propagation();

                        submit().await;
                    },
                    input {
                        type: "text",
                        class: "bg-input p-2 rounded",
                        placeholder: "Authenticator code",
                        value: "{code}",
                        oninput: move |e| code.set(e.value()),
                    }

                    if let Some(error) = error() {
                        p { class: "text-destructive", "{error}" }
                    }

                    div {
                        class: "flex justify-end w-full",
                        button { class: "bg-primary text-primary-foreground rounded px-4 py-2 hover:bg-primary/90 transition", "Register" }
                    }
                }
            }
        }
    }
}
