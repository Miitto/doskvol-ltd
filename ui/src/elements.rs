use dioxus::prelude::*;
use types::description::{Description as DT, Segment};

#[component]
pub fn Description<T: std::fmt::Display + 'static + PartialEq>(desc: DT<T>) -> Element {
    let segments = desc.to_segments();

    rsx! {
        p {
            for seg in segments {
                if let Segment::Text(text) = seg {
                    span { "{text}" }
                } else if let Segment::Italic(text) = seg {
                    span { class: "italic", "{text}" }
                } else if let Segment::Bold(text) = seg {
                    span { class: "font-bold", "{text}" }
                }
            }
        }
    }
}
