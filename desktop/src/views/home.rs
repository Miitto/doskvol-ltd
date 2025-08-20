use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let character = use_server_future(async move || api::get_character(0).await.unwrap())?;
    let character = if let Some(c) = character.value()() {
        use_signal(|| c)
    } else {
        return rsx! { div { "Loading..." } };
    };

    rsx! {
        ui::Character { character: character }
    }
}
