#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CrewPreview {
    pub id: crate::CrewId,
    pub name: String,
    pub specialty: CrewSpecialty,
    pub dm_name: String,
    pub player_count: usize,
}

#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Crew {
    pub id: crate::CrewId,
    pub name: String,
    pub specialty: CrewSpecialty,
    pub dm_id: crate::UserId,
}

#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(feature = "server", diesel(sql_type = diesel::sql_types::Text))]
pub enum CrewSpecialty {
    Assassins,
    Bravos,
    Cult,
    Hawkers,
    Smugglers,
    Shadows,
}

impl CrewSpecialty {
    pub const ALL: [CrewSpecialty; 6] = [
        CrewSpecialty::Assassins,
        CrewSpecialty::Bravos,
        CrewSpecialty::Cult,
        CrewSpecialty::Hawkers,
        CrewSpecialty::Smugglers,
        CrewSpecialty::Shadows,
    ];
}

impl std::fmt::Display for CrewSpecialty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrewSpecialty::Assassins => write!(f, "Assassins"),
            CrewSpecialty::Bravos => write!(f, "Bravos"),
            CrewSpecialty::Cult => write!(f, "Cult"),
            CrewSpecialty::Hawkers => write!(f, "Hawkers"),
            CrewSpecialty::Smugglers => write!(f, "Smugglers"),
            CrewSpecialty::Shadows => write!(f, "Shadows"),
        }
    }
}

impl std::str::FromStr for CrewSpecialty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "assassins" => Ok(CrewSpecialty::Assassins),
            "bravos" => Ok(CrewSpecialty::Bravos),
            "cult" => Ok(CrewSpecialty::Cult),
            "hawkers" => Ok(CrewSpecialty::Hawkers),
            "smugglers" => Ok(CrewSpecialty::Smugglers),
            "shadows" => Ok(CrewSpecialty::Shadows),
            _ => Err(format!("Invalid crew specialty: {s}")),
        }
    }
}

#[cfg(feature = "server")]
mod server {
    use super::*;
    use diesel::{
        backend::Backend,
        deserialize::FromSql,
        serialize::{Output, ToSql},
        sqlite::Sqlite,
    };

    impl ToSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for CrewSpecialty {
        fn to_sql<'a>(
            &'a self,
            out: &mut Output<'a, '_, diesel::sqlite::Sqlite>,
        ) -> diesel::serialize::Result {
            out.set_value(self.to_string());
            Ok(diesel::serialize::IsNull::No)
        }
    }

    impl FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for CrewSpecialty {
        fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            let s = <String as FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite>>::from_sql(
                bytes,
            )?;

            Ok(s.parse()?)
        }
    }
}
