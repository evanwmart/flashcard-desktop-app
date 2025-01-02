mod db;

/*********************************************
 * BUCKETS
 *********************************************/
#[tauri::command]
async fn create_bucket_command(name: String, interval_days: f64) -> Result<(), String> {
    db::buckets::create_bucket(name, interval_days)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn read_buckets_command() -> Result<Vec<(i32, String, f64)>, String> {
    db::buckets::read_buckets()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_bucket_command(id: i32, name: Option<String>, interval_days: Option<f64>) -> Result<(), String> {
    db::buckets::update_bucket(id, name, interval_days)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_bucket_command(id: i32) -> Result<(), String> {
    db::buckets::delete_bucket(id)
        .await
        .map_err(|e| e.to_string())
}

/*********************************************
 * DECKS
 *********************************************/
#[tauri::command]
async fn create_deck_command(name: String, created_date: String, style: Option<String>) -> Result<(), String> {
    db::decks::create_deck(name, created_date, style)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn read_decks_command() -> Result<Vec<(i32, String, String, Option<String>)>, String> {
    db::decks::read_decks()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_deck_command(
    id: i32,
    name: Option<String>,
    created_date: Option<String>,
    style: Option<String>,
) -> Result<(), String> {
    db::decks::update_deck(id, name, created_date, style)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_deck_command(id: i32) -> Result<(), String> {
    db::decks::delete_deck(id)
        .await
        .map_err(|e| e.to_string())
}

/*********************************************
 * FLASHCARDS
 *********************************************/
#[tauri::command]
async fn create_flashcard_command(
    deck_id: i32,
    html_front: String,
    html_back: String,
    bucket_id: i32,
    topics: Option<String>,
    priority: Option<i32>,
    delay_time: Option<f64>,
) -> Result<(), String> {
    db::flashcards::create_flashcard(deck_id, html_front, html_back, bucket_id, topics, priority, delay_time)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn read_flashcards_command(is_unlocked: bool) -> Result<Vec<(i32, String, String)>, String> {
    db::flashcards::read_flashcards(is_unlocked)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_flashcard_command(
    id: i32,
    html_front: Option<String>,
    html_back: Option<String>,
    bucket_id: Option<i32>,
    topics: Option<String>,
    priority: Option<i32>,
    delay_time: Option<f64>,
    is_unlocked: Option<bool>,
) -> Result<(), String> {
    db::flashcards::update_flashcard(
        id,
        html_front,
        html_back,
        bucket_id,
        topics,
        priority,
        delay_time,
        is_unlocked,
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_flashcard_command(id: i32) -> Result<(), String> {
    db::flashcards::delete_flashcard(id)
        .await
        .map_err(|e| e.to_string())
}

/*********************************************
 * MEDIA
 *********************************************/
#[tauri::command]
async fn create_media_command(
    flashcard_id: Option<i32>,
    media_type: String,
    file_path: String,
    description: Option<String>,
    batch_id: Option<i32>,
    duration: Option<f64>,
) -> Result<(), String> {
    db::media::create_media(flashcard_id, media_type, file_path, description, batch_id, duration)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn read_media_command() -> Result<Vec<(i32, Option<i32>, String, String, Option<String>, Option<i32>, Option<f64>)>, String> {
    db::media::read_media()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_media_command(
    id: i32,
    flashcard_id: Option<i32>,
    media_type: Option<String>,
    file_path: Option<String>,
    description: Option<String>,
    batch_id: Option<i32>,
    duration: Option<f64>,
) -> Result<(), String> {
    db::media::update_media(
        id,
        flashcard_id,
        media_type,
        file_path,
        description,
        batch_id,
        duration
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_media_command(id: i32) -> Result<(), String> {
    db::media::delete_media(id)
        .await
        .map_err(|e| e.to_string())
}

/*********************************************
 * SESSIONS
 *********************************************/
#[tauri::command]
async fn create_session_command(
    deck_id: i32,
    start_time: f64,
    duration: f64,
    cards_reviewed: i32,
    cards_improved: i32,
    cards_failed: i32,
    looked_at_count: i32,
) -> Result<(), String> {
    db::sessions::create_session(deck_id, start_time, duration, cards_reviewed, cards_improved, cards_failed, looked_at_count)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn read_sessions_command() -> Result<Vec<(i32, i32, f64, f64, i32, i32, i32, i32)>, String> {
    db::sessions::read_sessions()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_session_command(
    id: i32,
    duration: Option<f64>,
    cards_reviewed: Option<i32>,
    cards_improved: Option<i32>,
    cards_failed: Option<i32>,
    looked_at_count: Option<i32>,
) -> Result<(), String> {
    db::sessions::update_session(
        id,
        duration,
        cards_reviewed,
        cards_improved,
        cards_failed,
        looked_at_count
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_session_command(id: i32) -> Result<(), String> {
    db::sessions::delete_session(id)
        .await
        .map_err(|e| e.to_string())
}

/*********************************************
 * SETTINGS
 *********************************************/
#[tauri::command]
async fn create_setting_command(key: String, value: String) -> Result<(), String> {
    db::settings::create_setting(key, value)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn read_settings_command() -> Result<Vec<(String, String)>, String> {
    db::settings::read_settings()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_setting_command(key: String, value: String) -> Result<(), String> {
    db::settings::update_setting(key, value)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_setting_command(key: String) -> Result<(), String> {
    db::settings::delete_setting(key)
        .await
        .map_err(|e| e.to_string())
}

/*********************************************
 * TOPICS
 *********************************************/
#[tauri::command]
async fn create_topic_command(name: String, deck_id: i32) -> Result<(), String> {
    db::topics::create_topic(name, deck_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn read_topics_command() -> Result<Vec<(i32, String, i32)>, String> {
    db::topics::read_topics()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_topic_command(id: i32, name: Option<String>, deck_id: Option<i32>) -> Result<(), String> {
    db::topics::update_topic(id, name, deck_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_topic_command(id: i32) -> Result<(), String> {
    db::topics::delete_topic(id)
        .await
        .map_err(|e| e.to_string())
}

/*********************************************
 * MAIN Tauri APP ENTRY
 *********************************************/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_| {
            // Initialize the database asynchronously
            tauri::async_runtime::spawn(async {
                if let Err(err) = db::init_db().await {
                    eprintln!("Failed to initialize database: {:?}", err);
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Buckets
            create_bucket_command,
            read_buckets_command,
            update_bucket_command,
            delete_bucket_command,

            // Decks
            create_deck_command,
            read_decks_command,
            update_deck_command,
            delete_deck_command,

            // Flashcards
            create_flashcard_command,
            read_flashcards_command,
            update_flashcard_command,
            delete_flashcard_command,

            // Media
            create_media_command,
            read_media_command,
            update_media_command,
            delete_media_command,

            // Sessions
            create_session_command,
            read_sessions_command,
            update_session_command,
            delete_session_command,

            // Settings
            create_setting_command,
            read_settings_command,
            update_setting_command,
            delete_setting_command,

            // Topics
            create_topic_command,
            read_topics_command,
            update_topic_command,
            delete_topic_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
