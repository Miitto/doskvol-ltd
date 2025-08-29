//! This crate contains all shared fullstack server functions.
use dioxus::prelude::{server_fn::error::NoCustomError, *};

#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg(feature = "server")]
use db::schema::*;

#[cfg(feature = "server")]
mod db;

#[cfg(feature = "server")]
fn launch_server() {
    tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(async {});
}

#[server(GetCharacter)]
pub async fn get_character(id: i32) -> Result<types::Character, ServerFnError> {
    let mut conn = db::connect();

    let character: db::models::Character = db::schema::characters::table
        .find(id)
        .select(db::models::Character::as_select())
        .first(&mut conn)
        .map_err(|e| ServerFnError::WrappedServerError(NoCustomError))?;

    let harm = db::models::CharacterHarm::belonging_to(&character)
        .select(db::models::CharacterHarm::as_select())
        .load(&mut conn)
        .map_err(|e| ServerFnError::WrappedServerError(NoCustomError))?;

    let abilities = db::models::CharacterAbility::belonging_to(&character)
        .select(db::models::CharacterAbility::as_select())
        .load(&mut conn)
        .map_err(|e| ServerFnError::WrappedServerError(NoCustomError))?;

    todo!()
}

#[server(GetCrewCharacters)]
pub async fn get_crew_characters(
    crew_id: i64,
) -> Result<Vec<types::CharacterPreview>, ServerFnError> {
    todo!()
}

#[server(GetCrew)]
pub async fn get_crew(id: i64) -> Result<types::Crew, ServerFnError> {
    todo!()
}

#[server(GetAllCrews)]
pub async fn get_all_crews() -> Result<Vec<types::CrewPreview>, ServerFnError> {
    todo!()
}

#[server(CreateCrew)]
pub async fn create_crew(crew: db::models::NewCrew) -> Result<types::Crew, ServerFnError> {
    todo!()
}

#[server(CreateCharacter)]
pub async fn create_character(
    character: db::models::NewCharacter,
) -> Result<types::Character, ServerFnError> {
    todo!()
}
