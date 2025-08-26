//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod db;

#[server(GetCharacter)]
pub async fn get_character(id: usize) -> Result<types::Character, ServerFnError> {
    let character = db::query_character(id)?.ok_or_else(|| {
        ServerFnError::<server_fn::error::NoCustomError>::Request("Character not found".to_string())
    })?;

    Ok(character)
}

#[server(GetCrewCharacters)]
pub async fn get_crew_characters(
    crew_id: usize,
) -> Result<Vec<types::CharacterPreview>, ServerFnError> {
    let characters = db::query_crew_characters(crew_id)?;

    Ok(characters)
}

#[server(GetCrew)]
pub async fn get_crew(id: usize) -> Result<types::Crew, ServerFnError> {
    let crew = db::get_crew(id)?.ok_or_else(|| {
        ServerFnError::<server_fn::error::NoCustomError>::Request("Crew not found".to_string())
    })?;

    Ok(crew)
}

#[server(GetAllCrews)]
pub async fn get_all_crews() -> Result<Vec<types::CrewPreview>, ServerFnError> {
    let crews = db::query_crews()?;

    Ok(crews)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CrewCreate {
    pub name: String,
}

#[server(CreateCrew)]
pub async fn create_crew(crew: CrewCreate) -> Result<types::Crew, ServerFnError> {
    let crew = db::create_crew(crew)?;

    Ok(crew)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterCreate {
    pub crew_id: usize,
    pub name: String,
    pub class: types::Class,
}

#[server(CreateCharacter)]
pub async fn create_character(
    character: CharacterCreate,
) -> Result<types::Character, ServerFnError> {
    let character = db::create_character(character)?;

    Ok(character)
}
