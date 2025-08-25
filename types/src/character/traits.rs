#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Heritage {
    Akoros,
    TheDaggerIsles,
    Iruvia,
    Severos,
    Skovlan,
    Tycsheros,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Background {
    Academic,
    Labor,
    Law,
    Trade,
    Military,
    Noble,
    Underworld,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Vice {
    Faith,
    Gambling,
    Luxury,
    Obligation,
    Pleasure,
    Stupor,
    Weird,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Trauma {
    Cold,
    Haunted,
    Obsessed,
    Paranoid,
    Relentless,
    Soft,
    Unstable,
    Vicious,
}

bitflags::bitflags! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TraumaFlags: u8 {
    const COLD = 0b00000001;
    const HAUNTED = 0b00000010;
    const OBSESSED = 0b00000100;
    const PARANOID = 0b00001000;
    const RELENTLESS = 0b00010000;
    const SOFT = 0b00100000;
    const UNSTABLE = 0b01000000;
    const VICIOUS = 0b10000000;
}
}

impl Heritage {
    pub const ALL: [Heritage; 6] = [
        Heritage::Akoros,
        Heritage::TheDaggerIsles,
        Heritage::Iruvia,
        Heritage::Severos,
        Heritage::Skovlan,
        Heritage::Tycsheros,
    ];
}

impl std::fmt::Display for Heritage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heritage::Akoros => write!(f, "Akoros"),
            Heritage::TheDaggerIsles => write!(f, "The Dagger Isles"),
            Heritage::Iruvia => write!(f, "Iruvia"),
            Heritage::Severos => write!(f, "Severos"),
            Heritage::Skovlan => write!(f, "Skovlan"),
            Heritage::Tycsheros => write!(f, "Tycheros"),
        }
    }
}

impl From<String> for Heritage {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Akoros" => Heritage::Akoros,
            "The Dagger Isles" => Heritage::TheDaggerIsles,
            "Iruvia" => Heritage::Iruvia,
            "Severos" => Heritage::Severos,
            "Skovlan" => Heritage::Skovlan,
            "Tycheros" => Heritage::Tycsheros,
            _ => panic!("Unknown heritage: {s}"),
        }
    }
}

impl Background {
    pub const ALL: [Background; 7] = [
        Background::Academic,
        Background::Labor,
        Background::Law,
        Background::Trade,
        Background::Military,
        Background::Noble,
        Background::Underworld,
    ];
}

impl std::fmt::Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Background::Academic => write!(f, "Academic"),
            Background::Labor => write!(f, "Labor"),
            Background::Law => write!(f, "Law"),
            Background::Trade => write!(f, "Trade"),
            Background::Military => write!(f, "Military"),
            Background::Noble => write!(f, "Noble"),
            Background::Underworld => write!(f, "Underworld"),
        }
    }
}

impl From<String> for Background {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Academic" => Background::Academic,
            "Labor" => Background::Labor,
            "Law" => Background::Law,
            "Trade" => Background::Trade,
            "Military" => Background::Military,
            "Noble" => Background::Noble,
            "Underworld" => Background::Underworld,
            _ => panic!("Unknown background: {s}"),
        }
    }
}

impl Vice {
    pub const ALL: [Vice; 7] = [
        Vice::Faith,
        Vice::Gambling,
        Vice::Luxury,
        Vice::Obligation,
        Vice::Pleasure,
        Vice::Stupor,
        Vice::Weird,
    ];
}

impl std::fmt::Display for Vice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vice::Faith => write!(f, "Faith"),
            Vice::Gambling => write!(f, "Gambling"),
            Vice::Luxury => write!(f, "Luxury"),
            Vice::Obligation => write!(f, "Obligation"),
            Vice::Pleasure => write!(f, "Pleasure"),
            Vice::Stupor => write!(f, "Stupor"),
            Vice::Weird => write!(f, "Weird"),
        }
    }
}

impl From<String> for Vice {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Faith" => Vice::Faith,
            "Gambling" => Vice::Gambling,
            "Luxury" => Vice::Luxury,
            "Obligation" => Vice::Obligation,
            "Pleasure" => Vice::Pleasure,
            "Stupor" => Vice::Stupor,
            "Weird" => Vice::Weird,
            _ => panic!("Unknown vice: {s}"),
        }
    }
}

impl Trauma {
    pub const ALL: [Trauma; 8] = [
        Trauma::Cold,
        Trauma::Haunted,
        Trauma::Obsessed,
        Trauma::Paranoid,
        Trauma::Relentless,
        Trauma::Soft,
        Trauma::Unstable,
        Trauma::Vicious,
    ];
}

impl From<Trauma> for TraumaFlags {
    fn from(trauma: Trauma) -> Self {
        match trauma {
            Trauma::Cold => TraumaFlags::COLD,
            Trauma::Haunted => TraumaFlags::HAUNTED,
            Trauma::Obsessed => TraumaFlags::OBSESSED,
            Trauma::Paranoid => TraumaFlags::PARANOID,
            Trauma::Relentless => TraumaFlags::RELENTLESS,
            Trauma::Soft => TraumaFlags::SOFT,
            Trauma::Unstable => TraumaFlags::UNSTABLE,
            Trauma::Vicious => TraumaFlags::VICIOUS,
        }
    }
}

impl std::fmt::Display for Trauma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trauma::Cold => write!(f, "Cold"),
            Trauma::Haunted => write!(f, "Haunted"),
            Trauma::Obsessed => write!(f, "Obsessed"),
            Trauma::Paranoid => write!(f, "Paranoid"),
            Trauma::Relentless => write!(f, "Relentless"),
            Trauma::Soft => write!(f, "Soft"),
            Trauma::Unstable => write!(f, "Unstable"),
            Trauma::Vicious => write!(f, "Vicious"),
        }
    }
}

#[cfg(feature = "server")]
mod server {
    use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

    impl FromSql for super::Heritage {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| {
                Ok(match s.as_str() {
                    "Akoros" => super::Heritage::Akoros,
                    "The Dagger Isles" => super::Heritage::TheDaggerIsles,
                    "Iruvia" => super::Heritage::Iruvia,
                    "Severos" => super::Heritage::Severos,
                    "Skovlan" => super::Heritage::Skovlan,
                    "Tycheros" => super::Heritage::Tycsheros,
                    _ => return Err(FromSqlError::InvalidType),
                })
            })
        }
    }

    impl ToSql for super::Heritage {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let s = match self {
                super::Heritage::Akoros => "Akoros",
                super::Heritage::TheDaggerIsles => "The Dagger Isles",
                super::Heritage::Iruvia => "Iruvia",
                super::Heritage::Severos => "Severos",
                super::Heritage::Skovlan => "Skovlan",
                super::Heritage::Tycsheros => "Tycheros",
            };
            Ok(s.into())
        }
    }

    impl FromSql for super::Background {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| {
                Ok(match s.as_str() {
                    "Academic" => super::Background::Academic,
                    "Labor" => super::Background::Labor,
                    "Law" => super::Background::Law,
                    "Trade" => super::Background::Trade,
                    "Military" => super::Background::Military,
                    "Noble" => super::Background::Noble,
                    "Underworld" => super::Background::Underworld,
                    _ => return Err(FromSqlError::InvalidType),
                })
            })
        }
    }

    impl ToSql for super::Background {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let s = match self {
                super::Background::Academic => "Academic",
                super::Background::Labor => "Labor",
                super::Background::Law => "Law",
                super::Background::Trade => "Trade",
                super::Background::Military => "Military",
                super::Background::Noble => "Noble",
                super::Background::Underworld => "Underworld",
            };
            Ok(s.into())
        }
    }

    impl FromSql for super::Vice {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| {
                Ok(match s.as_str() {
                    "Faith" => super::Vice::Faith,
                    "Gambling" => super::Vice::Gambling,
                    "Luxury" => super::Vice::Luxury,
                    "Obligation" => super::Vice::Obligation,
                    "Pleasure" => super::Vice::Pleasure,
                    "Stupor" => super::Vice::Stupor,
                    "Weird" => super::Vice::Weird,
                    _ => return Err(FromSqlError::InvalidType),
                })
            })
        }
    }

    impl ToSql for super::Vice {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let s = match self {
                super::Vice::Faith => "Faith",
                super::Vice::Gambling => "Gambling",
                super::Vice::Luxury => "Luxury",
                super::Vice::Obligation => "Obligation",
                super::Vice::Pleasure => "Pleasure",
                super::Vice::Stupor => "Stupor",
                super::Vice::Weird => "Weird",
            };
            Ok(s.into())
        }
    }

    impl FromSql for super::Trauma {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            String::column_result(value).and_then(|s| {
                Ok(match s.as_str() {
                    "Cold" => super::Trauma::Cold,
                    "Haunted" => super::Trauma::Haunted,
                    "Obsessed" => super::Trauma::Obsessed,
                    "Paranoid" => super::Trauma::Paranoid,
                    "Relentless" => super::Trauma::Relentless,
                    "Soft" => super::Trauma::Soft,
                    "Unstable" => super::Trauma::Unstable,
                    "Vicious" => super::Trauma::Vicious,
                    _ => return Err(FromSqlError::InvalidType),
                })
            })
        }
    }

    impl ToSql for super::Trauma {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            let s = match self {
                super::Trauma::Cold => "Cold",
                super::Trauma::Haunted => "Haunted",
                super::Trauma::Obsessed => "Obsessed",
                super::Trauma::Paranoid => "Paranoid",
                super::Trauma::Relentless => "Relentless",
                super::Trauma::Soft => "Soft",
                super::Trauma::Unstable => "Unstable",
                super::Trauma::Vicious => "Vicious",
            };
            Ok(s.into())
        }
    }

    impl FromSql for super::TraumaFlags {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            u8::column_result(value)
                .and_then(|bits| Self::from_bits(bits).ok_or(FromSqlError::InvalidType))
        }
    }

    impl ToSql for super::TraumaFlags {
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            Ok(self.bits().into())
        }
    }
}
