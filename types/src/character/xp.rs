#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct XP {
    pub playbook: u8,
    pub insight: u8,
    pub prowess: u8,
    pub resolve: u8,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Dots {
    pub hunt: u8,
    pub study: u8,
    pub survey: u8,
    pub tinker: u8,
    pub finesse: u8,
    pub prowl: u8,
    pub skirmish: u8,
    pub wreck: u8,
    pub attune: u8,
    pub command: u8,
    pub consort: u8,
    pub sway: u8,
}

#[cfg(feature = "server")]
mod server {
    use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

    impl FromSql for super::XP {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() != 4 {
                    return Err(FromSqlError::InvalidType);
                }
                Ok(super::XP {
                    playbook: parts[0].parse().map_err(|_| FromSqlError::InvalidType)?,
                    insight: parts[1].parse().map_err(|_| FromSqlError::InvalidType)?,
                    prowess: parts[2].parse().map_err(|_| FromSqlError::InvalidType)?,
                    resolve: parts[3].parse().map_err(|_| FromSqlError::InvalidType)?,
                })
            })
        }
    }

    impl ToSql for super::XP {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let s = format!(
                "{},{},{},{}",
                self.playbook, self.insight, self.prowess, self.resolve
            );
            Ok(s.into())
        }
    }

    impl FromSql for super::Dots {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() != 12 {
                    return Err(FromSqlError::InvalidType);
                }
                Ok(super::Dots {
                    hunt: parts[0].parse().map_err(|_| FromSqlError::InvalidType)?,
                    study: parts[1].parse().map_err(|_| FromSqlError::InvalidType)?,
                    survey: parts[2].parse().map_err(|_| FromSqlError::InvalidType)?,
                    tinker: parts[3].parse().map_err(|_| FromSqlError::InvalidType)?,
                    finesse: parts[4].parse().map_err(|_| FromSqlError::InvalidType)?,
                    prowl: parts[5].parse().map_err(|_| FromSqlError::InvalidType)?,
                    skirmish: parts[6].parse().map_err(|_| FromSqlError::InvalidType)?,
                    wreck: parts[7].parse().map_err(|_| FromSqlError::InvalidType)?,
                    attune: parts[8].parse().map_err(|_| FromSqlError::InvalidType)?,
                    command: parts[9].parse().map_err(|_| FromSqlError::InvalidType)?,
                    consort: parts[10].parse().map_err(|_| FromSqlError::InvalidType)?,
                    sway: parts[11].parse().map_err(|_| FromSqlError::InvalidType)?,
                })
            })
        }
    }

    impl ToSql for super::Dots {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let s = format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}",
                self.hunt,
                self.study,
                self.survey,
                self.tinker,
                self.finesse,
                self.prowl,
                self.skirmish,
                self.wreck,
                self.attune,
                self.command,
                self.consort,
                self.sway
            );
            Ok(s.into())
        }
    }
}
