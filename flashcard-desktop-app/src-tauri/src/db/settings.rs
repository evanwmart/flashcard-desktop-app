use rusqlite::params;
use super::{DB_CONNECTION, DatabaseError};

/// CREATE
pub async fn create_setting(key: String, value: String) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2)",
                params![key, value],
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
pub async fn read_settings() -> Result<Vec<(String, String)>, DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(|conn| {
            let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
            let rows = stmt
                .query_map([], |row| {
                    Ok((row.get(0)?, row.get(1)?))
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
pub async fn update_setting(key: String, value: String) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "UPDATE settings
                 SET value = ?1
                 WHERE key = ?2",
                params![value, key],
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
pub async fn delete_setting(key: String) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "DELETE FROM settings WHERE key = ?1",
                params![key],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}
