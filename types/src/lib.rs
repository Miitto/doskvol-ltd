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
            _ => return Err(format!("Invalid class: {}", s)),
        })
    }
}

#[cfg(feature = "server")]
mod server {
    use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

    impl FromSql for super::Class {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value)
                .and_then(|s| s.as_str().try_into().map_err(|_| FromSqlError::InvalidType))
        }
    }

    impl ToSql for super::Class {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let s = match self {
                super::Class::Cutter => "Cutter",
                super::Class::Hound => "Hound",
                super::Class::Leech => "Leech",
                super::Class::Lurk => "Lurk",
                super::Class::Slide => "Slide",
                super::Class::Spider => "Spider",
                super::Class::Whisper => "Whisper",
            };
            Ok(s.into())
        }
    }
}
