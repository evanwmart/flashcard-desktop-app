use rusqlite::{Connection, Result};

fn initialize_database() -> Result<()> {
    let conn = Connection::open("flashcards.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS decks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS flashcards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            deck_id INTEGER NOT NULL,
            front_md TEXT NOT NULL,
            back_md TEXT NOT NULL,
            tags TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            last_reviewed TIMESTAMP,
            ease_factor REAL DEFAULT 2.5,
            interval INTEGER DEFAULT 1,
            next_review TIMESTAMP,
            FOREIGN KEY (deck_id) REFERENCES decks (id) ON DELETE CASCADE
        );",
        [],
    )?;

    Ok(())
}

#[tauri::command]
fn fetch_decks() -> Result<Vec<(i64, String, String)>, String> {
    let conn = Connection::open("flashcards.db").map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, created_at FROM decks")
        .map_err(|e| e.to_string())?;
    
    let decks_iter = stmt
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(|e| e.to_string())?;
    
    let decks = decks_iter.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    Ok(decks)
}

#[tauri::command]
fn insert_deck(name: String) -> Result<i64, String> {
    let conn = Connection::open("flashcards.db").map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO decks (name, created_at) VALUES (?1, CURRENT_TIMESTAMP)",
        &[&name],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = initialize_database();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_decks, insert_deck])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
