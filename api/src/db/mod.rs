#[cfg(feature = "server")]
use std::cell::RefCell;

use rusqlite::Connection;

mod character;
pub use character::*;
mod crew;
pub use crew::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: RefCell<Connection> = {
        let conn = Connection::open("blades.db").expect("Failed to open database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL
            );",
        ).expect("Failed to create users table");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS crews (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );",
        ).expect("Failed to create crews table");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS user_crews (
                user_id INTEGER NOT NULL,
                crew_id INTEGER NOT NULL,
                PRIMARY KEY (user_id, crew_id),
                FOREIGN KEY(user_id) REFERENCES users(id),
                FOREIGN KEY(crew_id) REFERENCES crews(id)
            );",
        ).expect("Failed to create user_crews table");

        conn.execute_batch(
            r#"CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY,
                player_id INTEGER NOT NULL,
                crew_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                class TEXT NOT NULL,
                look TEXT NOT NULL DEFAULT "",
                abilities TEXT NOT NULL DEFAULT "",
                heritage TEXT NOT NULL DEFAULT "Akoros",
                background TEXT NOT NULL DEFAULT "Academic",
                vice TEXT NOT NULL DEFAULT "Gambling",
                coin INTEGER NOT NULL DEFAULT 0,
                stash INTEGER NOT NULL DEFAULT 0,
                xp INTEGER NOT NULL DEFAULT "0,0,0,0",
                dots INTEGER NOT NULL DEFAULT "0,0,0,0,0,0,0,0,0,0,0,0",
                stress INTEGER NOT NULL DEFAULT 0,
                trauma INTEGER NOT NULL DEFAULT 0,
                harm INTEGER NOT NULL DEFAULT 0,
                healing INTEGER NOT NULL DEFAULT 0,
                armor INTEGER NOT NULL DEFAULT 0,
                notes TEXT NOT NULL DEFAULT "",
                contacts TEXT NOT NULL DEFAULT "<split>",
                class_items TEXT NOT NULL DEFAULT "",
                load TEXT,
                items TEXT NOT NULL DEFAULT "",
                FOREIGN KEY(player_id) REFERENCES users(id),
                FOREIGN KEY(crew_id) REFERENCES crews(id)
            );"#,
        ).expect("Failed to create characters table");

        RefCell::new(conn)
    }
}
