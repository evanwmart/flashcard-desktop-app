use rusqlite::{Connection, Result};
use flashcard_desktop_app::db::{initialize_database, insert_tag};

#[test]
fn test_database_initialization() -> Result<()> {
    let conn = initialize_database()?;
    // Check if the decks table exists
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
        println!("Error inserting tag in integration test: {:?}", e); // Debugging output
    }
    assert!(result.is_ok());

    // Verify the tag was inserted
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM tags WHERE name = ?1")?;
    let count: i64 = stmt.query_row(["Test Tag"], |row| row.get(0))?;
    assert_eq!(count, 1);

    Ok(())
}
