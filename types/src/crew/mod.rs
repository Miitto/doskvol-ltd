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
