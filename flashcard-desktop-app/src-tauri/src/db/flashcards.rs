use rusqlite::params;
use super::{DB_CONNECTION, DatabaseError};

pub async fn create_flashcard(
    deck_id: i32,
    html_front: String,
    html_back: String,
    bucket_id: i32,
    topics: Option<String>,
    priority: Option<i32>,
    delay_time: Option<f64>,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "INSERT INTO flashcards
                 (deck_id, html_front, html_back, bucket_id, topics, priority, delay_time)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![deck_id, html_front, html_back, bucket_id, topics, priority, delay_time],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

pub async fn read_flashcards(is_unlocked: bool) -> Result<Vec<(i32, String, String)>, DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, html_front, html_back
                 FROM flashcards
                 WHERE is_unlocked = ?1"
            )?;
            let rows = stmt
                .query_map(params![is_unlocked], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            Ok(rows)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

pub async fn update_flashcard(
    id: i32,
    html_front: Option<String>,
    html_back: Option<String>,
    bucket_id: Option<i32>,
    topics: Option<String>,
    priority: Option<i32>,
    delay_time: Option<f64>,
    is_unlocked: Option<bool>,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "UPDATE flashcards
                 SET html_front  = COALESCE(?1, html_front),
                     html_back   = COALESCE(?2, html_back),
                     bucket_id   = COALESCE(?3, bucket_id),
                     topics      = COALESCE(?4, topics),
                     priority    = COALESCE(?5, priority),
                     delay_time  = COALESCE(?6, delay_time),
                     is_unlocked = COALESCE(?7, is_unlocked)
                 WHERE id = ?8",
                params![
                    html_front,
                    html_back,
                    bucket_id,
                    topics,
                    priority,
                    delay_time,
                    is_unlocked,
                    id
                ],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

pub async fn delete_flashcard(id: i32) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "DELETE FROM flashcards WHERE id = ?1",
                params![id],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}
