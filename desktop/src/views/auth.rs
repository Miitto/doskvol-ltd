use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let nav = use_navigator();
    rsx! {
        ui::auth::Login {
            register: crate::Route::Register {}.into(),
            on_login: move || {
                nav.replace(crate::Route::Home {});
            },
        
        }
    }
}

#[component]
pub fn Register() -> Element {
    let nav = use_navigator();
    rsx! {
        ui::auth::Register {
            login: crate::Route::Login {}.into(),
            on_register: move || {
                nav.replace(crate::Route::Home {});
            },
        }
    }
}
