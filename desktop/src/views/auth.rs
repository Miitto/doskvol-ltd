use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        ui::auth::Login {
            register: crate::Route::Register {}.into()
        }
    }
}

#[component]
pub fn Register() -> Element {
    rsx! {
        ui::auth::Register {
            login: crate::Route::Login {}.into()
        }
    }
}
