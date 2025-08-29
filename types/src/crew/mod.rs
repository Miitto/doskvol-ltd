#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CrewPreview {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Crew {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(feature = "server", diesel(sql_type = diesel::sql_types::Text))]
pub enum CrewSpecialty {
    Adepts,
    Rooks,
    Rovers,
    Skulks,
    Thugs,
}
