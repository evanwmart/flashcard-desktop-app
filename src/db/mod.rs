use rusqlite::{Connection, Result, params};

// ===============================
// Database Initialization
// ===============================

/// Initializes the database connection and creates the necessary tables.
/// Returns a `Connection` instance to the SQLite database.
/// If `in_memory` is true, uses an in-memory database for testing.
pub fn initialize_database(in_memory: bool) -> Result<Connection> {
    let conn = if in_memory {
        Connection::open_in_memory()?
    } else {
        Connection::open("flashcard_app.db")?
    };

    // Create the `decks` table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS decks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            pack TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        "#,
        [],
    )?;

    // Create the `flashcards` table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS flashcards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            deck_id INTEGER NOT NULL,
            front_md TEXT NOT NULL,
            back_md TEXT NOT NULL,
            hint_a TEXT,
            hint_b TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY(deck_id) REFERENCES decks(id)
        );
        "#,
        [],
    )?;

    // Create the `relationships` table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS relationships (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_flashcard_id INTEGER NOT NULL,
            target_flashcard_id INTEGER NOT NULL,
            relationship_type TEXT NOT NULL,
            FOREIGN KEY(source_flashcard_id) REFERENCES flashcards(id),
            FOREIGN KEY(target_flashcard_id) REFERENCES flashcards(id)
        );
        "#,
        [],
    )?;

    // Create the `tags` and `flashcard_tags` tables
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );
        "#,
        [],
    )?;

    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS flashcard_tags (
            flashcard_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY(flashcard_id, tag_id),
            FOREIGN KEY(flashcard_id) REFERENCES flashcards(id),
            FOREIGN KEY(tag_id) REFERENCES tags(id)
        );
        "#,
        [],
    )?;

    // Create the `mindmap_nodes` table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS mindmap_nodes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            flashcard_id INTEGER NOT NULL,
            x_position REAL NOT NULL,
            y_position REAL NOT NULL,
            FOREIGN KEY(flashcard_id) REFERENCES flashcards(id)
        );
        "#,
        [],
    )?;

    Ok(conn)
}

// ===============================
// Tag Operations
// ===============================

/// Inserts a tag into the `tags` table.
pub fn insert_tag(conn: &Connection, name: &str) -> Result<usize> {
    conn.execute("INSERT INTO tags (name) VALUES (?1);", params![name])
        .map_err(|e| {
            println!("Error inserting tag: {:?}", e);
            e
        })
}

/// Associates a tag with a flashcard in the `flashcard_tags` table.
#[allow(dead_code)] // TODO
pub fn associate_tag(conn: &Connection, flashcard_id: i64, tag_id: i64) -> Result<usize> {
    conn.execute(
        "INSERT INTO flashcard_tags (flashcard_id, tag_id) VALUES (?1, ?2);",
        params![flashcard_id, tag_id],
    )
}

// ===============================
// Relationship and Mindmap Operations
// ===============================

/// Inserts a relationship between two flashcards in the `relationships` table.
#[allow(dead_code)] // TODO
pub fn insert_relationship(
    conn: &Connection,
    source_flashcard_id: i64,
    target_flashcard_id: i64,
    relationship_type: &str,
) -> Result<usize> {
    conn.execute(
        "INSERT INTO relationships (source_flashcard_id, target_flashcard_id, relationship_type) VALUES (?1, ?2, ?3);",
        params![source_flashcard_id, target_flashcard_id, relationship_type],
    )
}

/// Inserts a node in the `mindmap_nodes` table.
#[allow(dead_code)] // TODO
pub fn insert_mindmap_node(
    conn: &Connection,
    flashcard_id: i64,
    x_position: f64,
    y_position: f64,
) -> Result<usize> {
    conn.execute(
        "INSERT INTO mindmap_nodes (flashcard_id, x_position, y_position) VALUES (?1, ?2, ?3);",
        params![flashcard_id, x_position, y_position],
    )
}

// ===============================
// Deck Operations
// ===============================

/// Creates a new deck in the `decks` table.
#[allow(dead_code)] // TODO
pub fn create_deck(conn: &Connection, name: &str, pack: &str, created_at: &str) -> Result<usize> {
    conn.execute(
        "INSERT INTO decks (name, pack, created_at) VALUES (?1, ?2, ?3);",
        params![name, pack, created_at],
    )
}

/// Reads all decks from the `decks` table.
#[allow(dead_code)] // TODO
pub fn read_decks(conn: &Connection) -> Result<Vec<(i64, String, String, String)>> {
    let mut stmt = conn.prepare("SELECT id, name, pack, created_at FROM decks;")?;
    let rows: Vec<_> = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?
        .collect::<Result<_, _>>()?; // Collect results before `stmt` goes out of scope
    Ok(rows)
}

/// Updates a deck in the `decks` table.
#[allow(dead_code)] // TODO
pub fn update_deck(conn: &Connection, id: i64, name: &str, pack: &str) -> Result<usize> {
    conn.execute(
        "UPDATE decks SET name = ?1, pack = ?2 WHERE id = ?3;",
        params![name, pack, id],
    )
}

/// Deletes a deck from the `decks` table.
#[allow(dead_code)] // TODO
pub fn delete_deck(conn: &Connection, id: i64) -> Result<usize> {
    conn.execute("DELETE FROM decks WHERE id = ?1;", params![id])
}

// ===============================
// Flashcard Operations
// ===============================

/// Creates a new flashcard in the `flashcards` table.
#[allow(dead_code)] // TODO
pub fn create_flashcard(
    conn: &Connection,
    deck_id: i64,
    front_md: &str,
    back_md: &str,
    hint_a: Option<&str>,
    hint_b: Option<&str>,
    created_at: &str,
) -> Result<usize> {
    conn.execute(
        "INSERT INTO flashcards (deck_id, front_md, back_md, hint_a, hint_b, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        params![deck_id, front_md, back_md, hint_a, hint_b, created_at],
    )
}

/// Reads all flashcards for a specific deck.
#[allow(dead_code)] // TODO
pub fn read_flashcards_by_deck(
    conn: &Connection,
    deck_id: i64,
) -> Result<Vec<(i64, String, String, Option<String>, Option<String>, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, front_md, back_md, hint_a, hint_b, created_at FROM flashcards WHERE deck_id = ?1;",
    )?;
    let rows: Vec<_> = stmt
        .query_map(params![deck_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        })?
        .collect::<Result<_, _>>()?; // Collect results before `stmt` goes out of scope
    Ok(rows)
}

/// Updates a flashcard in the `flashcards` table.
#[allow(dead_code)] // TODO
pub fn update_flashcard(
    conn: &Connection,
    id: i64,
    front_md: &str,
    back_md: &str,
    hint_a: Option<&str>,
    hint_b: Option<&str>,
) -> Result<usize> {
    conn.execute(
        "UPDATE flashcards SET front_md = ?1, back_md = ?2, hint_a = ?3, hint_b = ?4 WHERE id = ?5;",
        params![front_md, back_md, hint_a, hint_b, id],
    )
}

/// Deletes a flashcard from the `flashcards` table.
#[allow(dead_code)] // TODO
pub fn delete_flashcard(conn: &Connection, id: i64) -> Result<usize> {
    conn.execute("DELETE FROM flashcards WHERE id = ?1;", params![id])
}

// ===============================
// Tests
// ===============================

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Result;

    #[test]
    fn test_database_initialization() -> Result<()> {
        let conn = initialize_database(false)?;
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='decks';")?;
        let exists: Option<String> = stmt.query_row([], |row| row.get(0)).ok();
        assert_eq!(exists, Some("decks".to_string()));
        Ok(())
    }

    #[test]
    fn test_insert_tag() -> Result<()> {
        let conn = initialize_database(false)?;
        conn.execute("DELETE FROM tags;", []).unwrap(); // Clear table
        assert!(insert_tag(&conn, "Test Tag").is_ok());
        Ok(())
    }
}
