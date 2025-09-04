#[cfg(feature = "server")]
use diesel::prelude::*;

pub mod models;
#[cfg(feature = "server")]
pub(crate) mod schema;

#[cfg(feature = "server")]
pub fn connect() -> diesel::SqliteConnection {
    if let Err(e) = dotenvy::dotenv() {
        dioxus::logger::tracing::error!("Failed to read .env file: {}", e);
        panic!("Failed to read .env file: {e}");
    }

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    diesel::SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
