mod bitflag_count;
pub use bitflag_count::BitCount;
pub mod description;
pub use crate::description::Description;
mod character;

pub use character::*;

mod crew;
pub use crew::*;

data::blades!();

impl TryFrom<&str> for Class {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, String> {
        Ok(match s.to_lowercase().as_str() {
            "cutter" => Class::Cutter,
            "hound" => Class::Hound,
            "leech" => Class::Leech,
            "lurk" => Class::Lurk,
            "slide" => Class::Slide,
            "spider" => Class::Spider,
            "whisper" => Class::Whisper,
            _ => return Err(format!("Invalid class: {s}")),
        })
    }
}
