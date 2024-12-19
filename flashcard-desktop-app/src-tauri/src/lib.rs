mod db;
mod commands;

use db::Database;
use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let database = runtime.block_on(Database::new("sqlite:flashcards.db"));

    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            create_deck,
            read_decks,
            update_deck,
            delete_deck,
            create_flashcard,
            read_flashcards,
            update_flashcard,
            delete_flashcard,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
