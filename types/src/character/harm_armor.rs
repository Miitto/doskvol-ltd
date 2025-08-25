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
mod server {
    use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

    impl FromSql for super::Harm {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| {
                let parts: Vec<&str> = s.split("<split>").collect();
                if parts.len() != 5 {
                    return Err(FromSqlError::InvalidType);
                }
                Ok(super::Harm(
                    [parts[0].to_string(), parts[1].to_string()],
                    [parts[2].to_string(), parts[3].to_string()],
                    parts[4].to_string(),
                ))
            })
        }
    }

    impl ToSql for super::Harm {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let s = format!(
                "{}<split>{}<split>{}<split>{}<split>{}",
                self.0[0], self.0[1], self.1[0], self.1[1], self.2
            );
            Ok(s.into())
        }
    }

    impl FromSql for super::ArmorFlags {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            u8::column_result(value)
                .and_then(|bits| Self::from_bits(bits).ok_or(FromSqlError::InvalidType))
        }
    }

    impl ToSql for super::ArmorFlags {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            Ok(self.bits().into())
        }
    }
}
