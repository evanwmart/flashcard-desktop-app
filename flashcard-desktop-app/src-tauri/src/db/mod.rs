pub mod buckets;
pub mod decks;
pub mod flashcards;
pub mod media;
pub mod settings;
pub mod sessions;
pub mod topics;

use tokio_rusqlite::{Connection, Result};
use tokio::sync::OnceCell;
use thiserror::Error;

// Shared database connection
pub static DB_CONNECTION: OnceCell<Connection> = OnceCell::const_new();

/// Custom error type for database operations
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection is not available")]
    ConnectionUnavailable,
    #[error(transparent)]
    TokioError(#[from] tokio_rusqlite::Error),
    // ^ This tells Rust how to turn a `tokio_rusqlite::Error` into `DatabaseError`.
}


// Initialize the database
pub async fn init_db() -> Result<()> {
    let conn = Connection::open("app.db").await?;

    conn.call(|conn| {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS buckets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                interval_days REAL NOT NULL
            );

            CREATE TABLE IF NOT EXISTS decks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_date TEXT NOT NULL,
                style TEXT NULL
            );

            CREATE TABLE IF NOT EXISTS flashcards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                deck_id INTEGER NOT NULL,
                html_front TEXT NOT NULL,
                html_back TEXT NOT NULL,
                bucket_id INTEGER NOT NULL,
                due_date REAL NULL,
                ease_factor REAL DEFAULT 2.5,
                priority INTEGER NULL,
                delay_time REAL NULL,
                last_reviewed REAL NULL,
                review_interval REAL DEFAULT 0,
                topics TEXT NULL,
                is_unlocked BOOLEAN DEFAULT 0,
                FOREIGN KEY (deck_id) REFERENCES decks(id),
                FOREIGN KEY (bucket_id) REFERENCES buckets(id)
            );

            CREATE TABLE IF NOT EXISTS prerequisites (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                flashcard_id INTEGER NOT NULL,
                prerequisite_id INTEGER NOT NULL,
                FOREIGN KEY (flashcard_id) REFERENCES flashcards(id),
                FOREIGN KEY (prerequisite_id) REFERENCES flashcards(id)
            );

            CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL UNIQUE,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS media (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                flashcard_id INTEGER NULL,
                type TEXT NOT NULL,
                file_path TEXT NOT NULL,
                description TEXT NULL,
                batch_id INTEGER NULL,
                duration REAL NULL,
                FOREIGN KEY (flashcard_id) REFERENCES flashcards(id)
            );

            CREATE TABLE IF NOT EXISTS topics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                deck_id INTEGER NOT NULL,
                FOREIGN KEY (deck_id) REFERENCES decks(id)
            );

            CREATE TABLE IF NOT EXISTS flashcard_topics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                flashcard_id INTEGER NOT NULL,
                topic_id INTEGER NOT NULL,
                FOREIGN KEY (flashcard_id) REFERENCES flashcards(id),
                FOREIGN KEY (topic_id) REFERENCES topics(id)
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                deck_id INTEGER NOT NULL,
                start_time REAL NOT NULL,
                duration REAL NOT NULL,
                cards_reviewed INTEGER NOT NULL,
                cards_improved INTEGER NOT NULL,
                cards_failed INTEGER NOT NULL DEFAULT 0,
                looked_at_count INTEGER NOT NULL,
                FOREIGN KEY (deck_id) REFERENCES decks(id)
            );
            "#
        )?;
        Ok(())
    }).await?;

    DB_CONNECTION.set(conn).unwrap();

    Ok(())
}
