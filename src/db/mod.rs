use rusqlite::{Connection, Result, params};


pub fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("flashcard_app.db")?;

    // Create `decks` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS decks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            pack TEXT NOT NULL,
            created_at TEXT NOT NULL
        );",
        [],
    )?;

    // Create `flashcards` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS flashcards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            deck_id INTEGER NOT NULL,
            front_md TEXT NOT NULL,
            back_md TEXT NOT NULL,
            hint_a TEXT,
            hint_b TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY(deck_id) REFERENCES decks(id)
        );",
        [],
    )?;

    // Create `relationships` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS relationships (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_flashcard_id INTEGER NOT NULL,
            target_flashcard_id INTEGER NOT NULL,
            relationship_type TEXT NOT NULL,
            FOREIGN KEY(source_flashcard_id) REFERENCES flashcards(id),
            FOREIGN KEY(target_flashcard_id) REFERENCES flashcards(id)
        );",
        [],
    )?;

    // Create `tags` and `flashcard_tags` tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS flashcard_tags (
            flashcard_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY(flashcard_id, tag_id),
            FOREIGN KEY(flashcard_id) REFERENCES flashcards(id),
            FOREIGN KEY(tag_id) REFERENCES tags(id)
        );",
        [],
    )?;

    // Create `mindmap_nodes` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mindmap_nodes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            flashcard_id INTEGER NOT NULL,
            x_position REAL NOT NULL,
            y_position REAL NOT NULL,
            FOREIGN KEY(flashcard_id) REFERENCES flashcards(id)
        );",
        [],
    )?;

    Ok(conn)
}

// Helper functions

pub fn insert_tag(conn: &Connection, name: &str) -> Result<usize> {
    conn.execute("INSERT INTO tags (name) VALUES (?1);", &[&name])
        .map_err(|e| {
            println!("Error inserting tag: {:?}", e);
            e
        })
}

pub fn associate_tag(conn: &Connection, flashcard_id: i64, tag_id: i64) -> Result<usize> {
    conn.execute(
        "INSERT INTO flashcard_tags (flashcard_id, tag_id) VALUES (?1, ?2);",
        &[&flashcard_id, &tag_id],
    )
}

pub fn insert_relationship(
    conn: &Connection,
    source_flashcard_id: i64,
    target_flashcard_id: i64,
    relationship_type: &str,
) -> Result<usize> {
    conn.execute(
        "INSERT INTO relationships (source_flashcard_id, target_flashcard_id, relationship_type) 
         VALUES (?1, ?2, ?3);",
        params![source_flashcard_id, target_flashcard_id, relationship_type],
    )
}

pub fn insert_mindmap_node(
    conn: &Connection,
    flashcard_id: i64,
    x_position: f64,
    y_position: f64,
) -> Result<usize> {
    conn.execute(
        "INSERT INTO mindmap_nodes (flashcard_id, x_position, y_position) 
         VALUES (?1, ?2, ?3);",
        params![flashcard_id, x_position, y_position],
    )
}

// Tests for the `db` module
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Result;

    #[test]
    fn test_database_initialization() -> Result<()> {
        let conn = initialize_database()?;
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='decks';")?;
        let exists: Option<String> = stmt.query_row([], |row| row.get(0)).ok();
        assert_eq!(exists, Some("decks".to_string()));
        Ok(())
    }


    #[test]
    fn test_insert_tag() -> Result<()> {
        // Initialize the database
        let conn = initialize_database()?;

        // Clear the `tags` table before running the test
        conn.execute("DELETE FROM tags;", []).expect("Failed to clear tags table");

        // Insert the tag
        let result = insert_tag(&conn, "Test Tag");
        if let Err(e) = &result {
            println!("Error in unit test insert_tag: {:?}", e); // Debugging output
        }
        assert!(result.is_ok());

        // Verify the tag was inserted
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM tags WHERE name = ?1")?;
        let count: i64 = stmt.query_row(["Test Tag"], |row| row.get(0))?;
        assert_eq!(count, 1);

        Ok(())
    }
}