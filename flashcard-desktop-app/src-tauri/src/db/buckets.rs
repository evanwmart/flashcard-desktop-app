use rusqlite::params;
use super::{DB_CONNECTION, DatabaseError};

pub async fn create_bucket(
    name: String,
    interval_days: f64,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "INSERT INTO buckets (name, interval_days) VALUES (?1, ?2)",
                params![name, interval_days],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

pub async fn read_buckets() -> Result<Vec<(i32, String, f64)>, DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(|conn| {
            let mut stmt = conn.prepare("SELECT id, name, interval_days FROM buckets")?;
            let rows = stmt
                .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            Ok(rows)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

pub async fn update_bucket(
    id: i32,
    name: Option<String>,
    interval_days: Option<f64>,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "UPDATE buckets 
                 SET name = COALESCE(?1, name), 
                     interval_days = COALESCE(?2, interval_days) 
                 WHERE id = ?3",
                params![name, interval_days, id],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

pub async fn delete_bucket(id: i32) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "DELETE FROM buckets WHERE id = ?1",
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