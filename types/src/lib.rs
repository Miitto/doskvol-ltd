mod bitflag_count;
pub use bitflag_count::BitCount;
pub mod description;
pub use crate::description::Description;
mod character;

pub use character::*;

data::blades!();
