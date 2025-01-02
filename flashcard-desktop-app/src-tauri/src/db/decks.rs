use rusqlite::params;
use super::{DB_CONNECTION, DatabaseError};

pub async fn create_deck(
    name: String,
    created_date: String,
    style: Option<String>,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "INSERT INTO decks (name, created_date, style) VALUES (?1, ?2, ?3)",
                params![name, created_date, style],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

pub async fn read_decks() -> Result<Vec<(i32, String, String, Option<String>)>, DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(|conn| {
            let mut stmt = conn.prepare("SELECT id, name, created_date, style FROM decks")?;
            let rows = stmt
                .query_map([], |row| {
                    Ok((
                        row.get(0)?,                       // id
                        row.get(1)?,                       // name
                        row.get(2)?,                       // created_date
                        row.get::<_, Option<String>>(3)?,  // style
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

pub async fn update_deck(
    id: i32,
    name: Option<String>,
    created_date: Option<String>,
    style: Option<String>,
) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "UPDATE decks
                 SET name         = COALESCE(?1, name),
                     created_date = COALESCE(?2, created_date),
                     style        = COALESCE(?3, style)
                 WHERE id = ?4",
                params![name, created_date, style, id],
            )
            .map(|_| ())?)
        })
        .await
        .map_err(DatabaseError::TokioError)
    } else {
        Err(DatabaseError::ConnectionUnavailable)
    }
}

pub async fn delete_deck(id: i32) -> Result<(), DatabaseError> {
    if let Some(conn) = DB_CONNECTION.get() {
        conn.call(move |conn| {
            Ok(conn.execute(
                "DELETE FROM decks WHERE id = ?1",
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
