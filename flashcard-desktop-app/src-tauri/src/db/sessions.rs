use rusqlite::params;
use super::{DB_CONNECTION, DatabaseError};

/// CREATE
pub async fn create_session(
    deck_id: i32,
    start_time: f64,
    duration: f64,
    cards_reviewed: i32,
    cards_improved: i32,
    cards_failed: i32,
    looked_at_count: i32,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "INSERT INTO sessions
                 (deck_id, start_time, duration, cards_reviewed, cards_improved, cards_failed, looked_at_count)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    deck_id,
                    start_time,
                    duration,
                    cards_reviewed,
                    cards_improved,
                    cards_failed,
                    looked_at_count
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

/// READ
pub async fn read_sessions() -> Result<Vec<(i32, i32, f64, f64, i32, i32, i32, i32)>, DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, deck_id, start_time, duration,
                        cards_reviewed, cards_improved, cards_failed, looked_at_count
                 FROM sessions"
            )?;

            let rows = stmt
                .query_map([], |row| {
                    Ok((
                        row.get(0)?, // id
                        row.get(1)?, // deck_id
                        row.get(2)?, // start_time
                        row.get(3)?, // duration
                        row.get(4)?, // cards_reviewed
                        row.get(5)?, // cards_improved
                        row.get(6)?, // cards_failed
                        row.get(7)?, // looked_at_count
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
pub async fn update_session(
    id: i32,
    duration: Option<f64>,
    cards_reviewed: Option<i32>,
    cards_improved: Option<i32>,
    cards_failed: Option<i32>,
    looked_at_count: Option<i32>,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "UPDATE sessions
                 SET duration        = COALESCE(?1, duration),
                     cards_reviewed   = COALESCE(?2, cards_reviewed),
                     cards_improved   = COALESCE(?3, cards_improved),
                     cards_failed     = COALESCE(?4, cards_failed),
                     looked_at_count  = COALESCE(?5, looked_at_count)
                 WHERE id = ?6",
                params![duration, cards_reviewed, cards_improved, cards_failed, looked_at_count, id],
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
pub async fn delete_session(id: i32) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "DELETE FROM sessions WHERE id = ?1",
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
