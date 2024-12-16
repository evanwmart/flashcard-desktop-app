use rusqlite::Result;
use flashcard_desktop_app::db::{
    initialize_database, insert_tag, create_deck, read_decks, create_flashcard, read_flashcards_by_deck,
};

/// Test the creation and reading of a deck in the database.
#[test]
fn test_create_and_read_deck() -> Result<()> {
    // Use an in-memory database for isolated testing.
    let conn = initialize_database(true)?;

    // Insert a new deck.
    create_deck(&conn, "Sample Deck", "Pack A", "2024-12-15")?;

    // Fetch all decks and verify the results.
    let decks = read_decks(&conn)?;
    assert_eq!(decks.len(), 1, "Expected 1 deck in the database.");
    assert_eq!(decks[0].1, "Sample Deck", "Deck name does not match.");
    Ok(())
}

/// Test the creation and reading of flashcards within a specific deck.
#[test]
fn test_create_and_read_flashcards() -> Result<()> {
    // Use an in-memory database for isolated testing.
    let conn = initialize_database(true)?;

    // Insert a new deck and retrieve its ID.
    let deck_id = create_deck(&conn, "Sample Deck", "Pack A", "2024-12-15")? as i64;

    // Insert a flashcard into the newly created deck.
    create_flashcard(
        &conn,
        deck_id,
        "Front of card",
        "Back of card",
        Some("Hint A"),
        Some("Hint B"),
        "2024-12-15",
    )?;

    // Fetch all flashcards in the deck and verify the results.
    let flashcards = read_flashcards_by_deck(&conn, deck_id)?;
    assert_eq!(flashcards.len(), 1, "Expected 1 flashcard in the deck.");
    assert_eq!(flashcards[0].1, "Front of card", "Flashcard front does not match.");
    Ok(())
}

/// Test database initialization and table creation.
#[test]
fn test_database_initialization() -> Result<()> {
    // Use an in-memory database for isolated testing.
    let conn = initialize_database(true)?;

    // Check if the `decks` table exists in the database.
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='decks';")?;
    let exists: Option<String> = stmt.query_row([], |row| row.get(0)).ok();

    assert_eq!(exists, Some("decks".to_string()), "Decks table was not created.");
    Ok(())
}

/// Test the insertion of a tag and its existence in the database.
#[test]
fn test_insert_tag() -> Result<()> {
    // Use an in-memory database for isolated testing.
    let conn = initialize_database(true)?;

    // Clear the `tags` table to ensure a clean test environment.
    conn.execute("DELETE FROM tags;", []).expect("Failed to clear tags table");

    // Insert a new tag into the database.
    let result = insert_tag(&conn, "Test Tag");
    if let Err(e) = &result {
        eprintln!("Error inserting tag in integration test: {:?}", e); // Debugging output for failures.
    }
    assert!(result.is_ok(), "Failed to insert tag.");

    // Verify the tag exists in the database.
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM tags WHERE name = ?1")?;
    let count: i64 = stmt.query_row(["Test Tag"], |row| row.get(0))?;
    assert_eq!(count, 1, "Tag was not inserted correctly.");

    Ok(())
}
