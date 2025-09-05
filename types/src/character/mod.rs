mod harm_armor;
mod traits;
mod xp;

pub use harm_armor::*;
pub use traits::*;
pub use xp::*;

use crate::{Class, Description};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CharacterPreview {
    pub id: crate::CharacterId,
    pub player_id: crate::UserId,
    pub player_name: String,
    pub crew_id: crate::CrewId,
    pub name: String,
    pub class: Class,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub id: crate::CharacterId,
    pub user_id: crate::UserId,
    pub crew_id: crate::CrewId,
    pub name: String,
    pub look: Description<String>,
    pub heritage: Heritage,
    pub background: Background,
    pub vice: Vice,
    pub stress: u8,
    pub trauma: TraumaFlags,
    pub harm: Harm,
    pub healing: u8,
    pub armor: ArmorFlags,
    pub notes: Description<String>,
    pub class: Class,
    pub abilities: Vec<String>,
    pub contacts: Contacts,
    pub class_items: Vec<String>,
    pub stash: u8,
    pub coin: u8,
    pub xp: XP,
    pub dots: Dots,
    pub load: Option<Load>,
    pub items: Items,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Contacts {
    pub friends: Vec<String>,
    pub rivals: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Load {
    Light,
    Medium,
    Heavy,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub struct Items: u16 {
        const BLADE = 0b0000_0000_0000_0001;
        const THROWING_KNIVES = 0b0000_0000_0000_0010;
        const PISTOL = 0b0000_0000_0000_0100;
        const PISTOL_2 = 0b0000_0000_0000_1000;
        const LARGE_WEAPON = 0b0000_0000_0001_0000;
        const UNUSUAL_WEAPON = 0b0000_0000_0010_0000;
        const ARMOR = 0b0000_0000_0100_0000;
        const HEAVY_ARMOR = 0b0000_0000_1000_0000;
        const BURGLARY_GEAR = 0b0000_0001_0000_0000;
        const CLIMBING_GEAR = 0b0000_0010_0000_0000;
        const ARCANE_IMPLEMENTS = 0b0000_0100_0000_0000;
        const DOCUMENTS = 0b0000_1000_0000_0000;
        const SUBTERFUGE_SUPPLIES = 0b0001_0000_0000_0000;
        const DEMO_TOOLS = 0b0010_0000_0000_0000;
        const TINKER_TOOLS = 0b0100_0000_0000_0000;
        const LANTERN = 0b1000_0000_0000_0000;
    }
}
