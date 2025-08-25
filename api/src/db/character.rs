use super::DB;
use rusqlite::Result;
pub fn query_character(id: usize) -> Result<Option<types::Character>> {
    DB.with(|conn| {
        let mut stmt = conn.prepare(
            "SELECT
        id, name, class, look, abilities, heritage,
        background, vice, coin, stash, xp, dots,
        stress, trauma, harm, healing, armor,
        notes, contacts, class_items, load,
        items, player_id, crew_id
        FROM characters WHERE id = ?1 LIMIT 1",
        )?;

        let mut character_iter = stmt.query_map([id], |row| {
            let abilities_str: String = row.get(4)?;
            let abilities = abilities_str
                .split("<split>")
                .map(|s| s.to_string())
                .collect();

            let class_items_str: String = row.get(19)?;
            let class_items = class_items_str
                .split("<split>")
                .map(|s| s.to_string())
                .collect();
            Ok(types::Character {
                id: row.get(0)?,
                player_id: row.get(22)?,
                crew_id: row.get(23)?,
                name: row.get(1)?,
                class: row.get(2)?,
                look: row.get(3)?,
                abilities,
                heritage: row.get(5)?,
                background: row.get(6)?,
                vice: row.get(7)?,
                coin: row.get(8)?,
                stash: row.get(9)?,
                xp: row.get(10)?,
                dots: row.get(11)?,
                stress: row.get(12)?,
                trauma: row.get(13)?,
                harm: row.get(14)?,
                healing: row.get(15)?,
                armor: row.get(16)?,
                notes: row.get(17)?,
                contacts: row.get(18)?,
                class_items,
                load: row.get(20)?,
                items: row.get(21)?,
            })
        })?;

        let next = character_iter.next();

        if let Some(character) = next {
            Ok(Some(character?))
        } else {
            Ok(None)
        }
    })
}
