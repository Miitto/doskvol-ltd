#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Harm(pub [String; 2], pub [String; 2], pub String);

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub struct ArmorFlags: u8 {
        const ARMOR = 0b001;
        const HEAVY = 0b010;
        const SPECIAL = 0b100;
    }
}

#[cfg(feature = "server")]
mod server {}
