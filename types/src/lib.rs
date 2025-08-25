mod bitflag_count;
pub use bitflag_count::BitCount;
pub mod description;
pub use crate::description::Description;
mod character;

pub use character::*;

mod crew;
pub use crew::*;

data::blades!();

#[cfg(feature = "server")]
mod server {
    use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

    impl FromSql for super::Class {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| match s.as_str() {
                "Cutter" => Ok(super::Class::Cutter),
                "Hound" => Ok(super::Class::Hound),
                "Leech" => Ok(super::Class::Leech),
                "Lurk" => Ok(super::Class::Lurk),
                "Slide" => Ok(super::Class::Slide),
                "Spider" => Ok(super::Class::Spider),
                "Whisper" => Ok(super::Class::Whisper),
                _ => Err(FromSqlError::InvalidType),
            })
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
