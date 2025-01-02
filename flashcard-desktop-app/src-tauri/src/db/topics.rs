use rusqlite::params;
use super::{DB_CONNECTION, DatabaseError};

/// CREATE
pub async fn create_topic(name: String, deck_id: i32) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "INSERT INTO topics (name, deck_id) VALUES (?1, ?2)",
                params![name, deck_id],
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
pub async fn read_topics() -> Result<Vec<(i32, String, i32)>, DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(|conn| {
            let mut stmt = conn.prepare("SELECT id, name, deck_id FROM topics")?;
            let rows = stmt
                .query_map([], |row| {
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

/// UPDATE
pub async fn update_topic(id: i32, name: Option<String>, deck_id: Option<i32>) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "UPDATE topics
                 SET name   = COALESCE(?1, name),
                     deck_id = COALESCE(?2, deck_id)
                 WHERE id = ?3",
                params![name, deck_id, id],
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
pub async fn delete_topic(id: i32) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "DELETE FROM topics WHERE id = ?1",
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
