use rusqlite::params;
use super::{DB_CONNECTION, DatabaseError};

/// CREATE
pub async fn create_media(
    flashcard_id: Option<i32>,
    media_type: String,
    file_path: String,
    description: Option<String>,
    batch_id: Option<i32>,
    duration: Option<f64>,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "INSERT INTO media 
                 (flashcard_id, type, file_path, description, batch_id, duration) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![flashcard_id, media_type, file_path, description, batch_id, duration],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

/// READ
pub async fn read_media() -> Result<Vec<(i32, Option<i32>, String, String, Option<String>, Option<i32>, Option<f64>)>, DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, flashcard_id, type, file_path, description, batch_id, duration
                 FROM media"
            )?;

            let rows = stmt
                .query_map([], |row| {
                    Ok((
                        row.get(0)?,                        // id
                        row.get::<_, Option<i32>>(1)?,      // flashcard_id
                        row.get(2)?,                        // type
                        row.get(3)?,                        // file_path
                        row.get::<_, Option<String>>(4)?,   // description
                        row.get::<_, Option<i32>>(5)?,      // batch_id
                        row.get::<_, Option<f64>>(6)?,      // duration
                    ))
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

/// UPDATE
pub async fn update_media(
    id: i32,
    flashcard_id: Option<i32>,
    media_type: Option<String>,
    file_path: Option<String>,
    description: Option<String>,
    batch_id: Option<i32>,
    duration: Option<f64>,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "UPDATE media 
                 SET flashcard_id = COALESCE(?1, flashcard_id),
                     type         = COALESCE(?2, type),
                     file_path    = COALESCE(?3, file_path),
                     description  = COALESCE(?4, description),
                     batch_id     = COALESCE(?5, batch_id),
                     duration     = COALESCE(?6, duration)
                 WHERE id = ?7",
                params![flashcard_id, media_type, file_path, description, batch_id, duration, id],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

/// DELETE
pub async fn delete_media(id: i32) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "DELETE FROM media WHERE id = ?1",
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
