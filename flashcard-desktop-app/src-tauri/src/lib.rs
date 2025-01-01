use tokio_rusqlite::{Connection, Result};
use tokio::sync::OnceCell;
use rusqlite::Error as RusqliteError; // Explicitly import rusqlite for error handling

#[tauri::command]
async fn greet(name: &str) -> std::result::Result<String, String> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[tauri::command]
async fn fetch_flashcards() -> std::result::Result<Vec<(i32, String, String)>, String> {
    if let Some(conn) = DB_CONNECTION.get() {
        let flashcards = conn
            .call(|conn| {
                let mut stmt = conn.prepare("SELECT id, html_front, html_back FROM flashcards")?;
                let rows = stmt
                    .query_map([], |row| {
                        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                    })?
                    .collect::<std::result::Result<Vec<_>, RusqliteError>>()?;
                Ok(rows)
            })
            .await;

        match flashcards {
            Ok(data) => Ok(data),
            Err(_) => Err("Failed to fetch flashcards.".to_string()),
        }
    } else {
        Err("Database connection is not available.".to_string())
    }
}

static DB_CONNECTION: OnceCell<Connection> = OnceCell::const_new();

pub async fn init_db() -> Result<()> {
    let conn = Connection::open("flashcards.db").await?;

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
                name TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS flashcards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                deck_id INTEGER NOT NULL,
                html_front TEXT NOT NULL,
                html_back TEXT NOT NULL,
                bucket_id INTEGER NOT NULL,
                due_date TEXT,
                ease_factor REAL DEFAULT 2.5,
                FOREIGN KEY (deck_id) REFERENCES decks(id),
                FOREIGN KEY (bucket_id) REFERENCES buckets(id)
            );

            CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL UNIQUE,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS media (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                flashcard_id INTEGER NOT NULL,
                type TEXT NOT NULL,
                file_path TEXT NOT NULL,
                description TEXT,
                FOREIGN KEY (flashcard_id) REFERENCES flashcards(id)
            );
            "#
        )?;
        Ok(())
    }).await?;

    DB_CONNECTION.set(conn).unwrap();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_| {
            tauri::async_runtime::spawn(async {
                if let Err(err) = init_db().await {
                    eprintln!("Failed to initialize database: {:?}", err);
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fetch_flashcards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
