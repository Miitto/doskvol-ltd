use rusqlite::Connection;

mod character;
pub use character::*;
mod crew;
pub use crew::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: Connection = {
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
            "CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY,
                player_id INTEGER NOT NULL,
                crew_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                class TEXT NOT NULL,
                look TEXT NOT NULL,
                abilities TEXT NOT NULL,
                heritage TEXT NOT NULL,
                background TEXT NOT NULL,
                vice TEXT NOT NULL,
                coin INTEGER NOT NULL,
                stash INTEGER NOT NULL,
                xp INTEGER NOT NULL,
                dots INTEGER NOT NULL,
                stress INTEGER NOT NULL,
                trauma INTEGER NOT NULL,
                harm INTEGER NOT NULL,
                healing INTEGER NOT NULL,
                armor INTEGER NOT NULL,
                notes TEXT NOT NULL,
                contacts TEXT NOT NULL,
                class_items TEXT NOT NULL,
                load TEXT NOT NULL,
                items TEXT NOT NULL,
                FOREIGN KEY(player_id) REFERENCES users(id),
                FOREIGN KEY(crew_id) REFERENCES crews(id)
            );",
        ).expect("Failed to create characters table");

        conn
    }
}
