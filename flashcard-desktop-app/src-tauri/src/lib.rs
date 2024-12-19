use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Database {
    pub pool: Arc<Mutex<SqlitePool>>,
}

impl Database {
    pub async fn new(database_url: &str) -> Self {
        let pool = SqlitePool::connect(database_url)
            .await
            .expect("Failed to connect to the database");

        // Apply schema or migrations
        sqlx::query(include_str!("../migrations/schema.sql"))
            .execute(&pool)
            .await
            .expect("Failed to initialize database schema");

        Self {
            pool: Arc::new(Mutex::new(pool)),
        }
    }
}

// Example command to interact with the database
#[tauri::command]
async fn create_deck(database: tauri::State<'_, Database>, name: String) -> Result<(), String> {
    let pool = database.pool.lock().await;
    sqlx::query("INSERT INTO decks (name) VALUES (?)")
        .bind(name)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(Database::new("sqlite://flashcards.db"));

    tauri::Builder::default()
        .manage(database) // Share database state with commands
        .invoke_handler(tauri::generate_handler![create_deck])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
