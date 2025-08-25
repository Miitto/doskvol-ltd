mod harm_armor;
mod traits;
mod xp;

pub use harm_armor::*;
pub use traits::*;
pub use xp::*;

use crate::{Class, Description};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CharacterPreview {
    pub id: usize,
    pub player_id: usize,
    pub crew_id: usize,
    pub name: String,
    pub class: Class,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub id: usize,
    pub player_id: usize,
    pub crew_id: usize,
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

#[cfg(feature = "server")]
mod server {
    use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

    impl FromSql for super::Contacts {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| {
                let parts: Vec<&str> = s.split("<split>").collect();
                if parts.len() != 2 {
                    return Err(FromSqlError::InvalidType);
                }
                let friends = if parts[0].is_empty() {
                    vec![]
                } else {
                    parts[0].split(',').map(|s| s.to_string()).collect()
                };
                let rivals = if parts[1].is_empty() {
                    vec![]
                } else {
                    parts[1].split(',').map(|s| s.to_string()).collect()
                };
                Ok(super::Contacts { friends, rivals })
            })
        }
    }

    impl ToSql for super::Contacts {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let friends = self.friends.join(",");
            let rivals = self.rivals.join(",");
            let s = format!("{friends}<split>{rivals}");
            Ok(s.into())
        }
    }

    impl FromSql for super::Load {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| match s.as_str() {
                "Light" => Ok(super::Load::Light),
                "Medium" => Ok(super::Load::Medium),
                "Heavy" => Ok(super::Load::Heavy),
                _ => Err(FromSqlError::InvalidType),
            })
        }
    }

    impl ToSql for super::Load {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let s = match self {
                super::Load::Light => "Light",
                super::Load::Medium => "Medium",
                super::Load::Heavy => "Heavy",
            };
            Ok(s.into())
        }
    }

    impl FromSql for super::Items {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            u16::column_result(value)
                .and_then(|bits| Self::from_bits(bits).ok_or(FromSqlError::InvalidType))
        }
    }

    impl ToSql for super::Items {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            Ok(self.bits().into())
        }
    }
}
