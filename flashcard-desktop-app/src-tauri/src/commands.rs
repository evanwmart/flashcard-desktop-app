use crate::db::Database;
use tauri::State;
use sqlx::Row;

// Create a deck
#[tauri::command]
pub async fn create_deck(database: State<'_, Database>, name: String) -> Result<i64, String> {
    let pool = database.pool.lock().await;
    let result = sqlx::query("INSERT INTO decks (name) VALUES (?)")
        .bind(name)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(result.last_insert_rowid())
}

// Read all decks
#[tauri::command]
pub async fn read_decks(database: State<'_, Database>) -> Result<Vec<(i64, String, String)>, String> {
    let pool = database.pool.lock().await;
    let rows = sqlx::query("SELECT id, name, created_at FROM decks")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    let decks = rows
        .iter()
        .map(|row| {
            (
                row.get::<i64, _>("id"),
                row.get::<String, _>("name"),
                row.get::<String, _>("created_at"),
            )
        })
        .collect();
    Ok(decks)
}

// Update a deck
#[tauri::command]
pub async fn update_deck(database: State<'_, Database>, id: i64, name: String) -> Result<(), String> {
    let pool = database.pool.lock().await;
    sqlx::query("UPDATE decks SET name = ? WHERE id = ?")
        .bind(name)
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Delete a deck
#[tauri::command]
pub async fn delete_deck(database: State<'_, Database>, id: i64) -> Result<(), String> {
    let pool = database.pool.lock().await;
    sqlx::query("DELETE FROM decks WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Create a flashcard
#[tauri::command]
pub async fn create_flashcard(
    database: State<'_, Database>,
    deck_id: i64,
    front_md: String,
    back_md: String,
    tags: Option<String>,
) -> Result<i64, String> {
    let pool = database.pool.lock().await;
    let result = sqlx::query(
        "INSERT INTO flashcards (deck_id, front_md, back_md, tags) VALUES (?, ?, ?, ?)",
    )
    .bind(deck_id)
    .bind(front_md)
    .bind(back_md)
    .bind(tags)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(result.last_insert_rowid())
}

// Read flashcards by deck
#[tauri::command]
pub async fn read_flashcards(
    database: State<'_, Database>,
    deck_id: i64,
) -> Result<Vec<(i64, String, String, Option<String>)>, String> {
    let pool = database.pool.lock().await;
    let rows = sqlx::query(
        "SELECT id, front_md, back_md, tags FROM flashcards WHERE deck_id = ?",
    )
    .bind(deck_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    let flashcards = rows
        .iter()
        .map(|row| {
            (
                row.get::<i64, _>("id"),
                row.get::<String, _>("front_md"),
                row.get::<String, _>("back_md"),
                row.get::<Option<String>, _>("tags"),
            )
        })
        .collect();
    Ok(flashcards)
}

// Update a flashcard
#[tauri::command]
pub async fn update_flashcard(
    database: State<'_, Database>,
    id: i64,
    front_md: String,
    back_md: String,
    tags: Option<String>,
) -> Result<(), String> {
    let pool = database.pool.lock().await;
    sqlx::query(
        "UPDATE flashcards SET front_md = ?, back_md = ?, tags = ? WHERE id = ?",
    )
    .bind(front_md)
    .bind(back_md)
    .bind(tags)
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

// Delete a flashcard
#[tauri::command]
pub async fn delete_flashcard(database: State<'_, Database>, id: i64) -> Result<(), String> {
    let pool = database.pool.lock().await;
    sqlx::query("DELETE FROM flashcards WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}