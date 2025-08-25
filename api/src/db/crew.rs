use super::DB;
use rusqlite::{Connection, Result};

pub fn query_crew_characters(crew_id: usize) -> Result<Vec<types::CharacterPreview>> {
    DB.with(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id, name, class, player_id, crew_id FROM characters WHERE crew_id = ?1",
        )?;

        let character_iter = stmt.query_map([crew_id], |row| {
            Ok(types::CharacterPreview {
                id: row.get(0)?,
                player_id: row.get(3)?,
                crew_id: row.get(4)?,
                name: row.get(1)?,
                class: row.get(2)?,
            })
        })?;

        let characters: Vec<types::CharacterPreview> =
            character_iter.filter_map(Result::ok).collect();

        Ok(characters)
    })
}

pub fn get_crew(id: usize) -> Result<Option<types::Crew>> {
    DB.with(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id, name
        FROM crews WHERE id = ?1 LIMIT 1",
        )?;

        let mut crew_iter = stmt.query_map([id], |row| {
            Ok(types::Crew {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let next = crew_iter.next();

        if let Some(crew) = next {
            Ok(Some(crew?))
        } else {
            Ok(None)
        }
    })
}

pub fn query_crews() -> Result<Vec<types::CrewPreview>> {
    DB.with(|conn| {
        let mut stmt = conn.prepare("SELECT id, name FROM crews")?;

        let crew_iter = stmt.query_map([], |row| {
            Ok(types::CrewPreview {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let crews: Vec<types::CrewPreview> = crew_iter.filter_map(Result::ok).collect();

        Ok(crews)
    })
}
