# flashcard-desktop-app


# Development Timeline: Flashcard App

## **Week 1: Initial MVP**
### **Goal**: Core flashcard functionality with Markdown rendering and SQLite setup.

- **Project Setup**
  - [X] Initialize the Rust project with the Iced framework.
  - Set up cross-platform build configuration.
  - [X] Windows
  - [X] Linux
  - [ ] MacOS TODO @ later date.

- **SQLite Integration**
  - [X] Add SQLite as the persistence layer.
  - [X] Create a database schema for decks and flashcards:
    - [X] Tables for decks (`id`, `name`, `created_at`).
    - [X] Tables for flashcards (`id`, `deck_id`, `front_md`, `back_md`, `created_at`).
  - [X] Write test scripts to validate SQLite schema creation.

- **Flashcard CRUD Operations**
  - [ ] Implement backend logic for creating, reading, updating, and deleting flashcards and decks.
  - [ ] Test:
    - [ ] Add a deck and verify it appears in the database.
    - [ ] Add flashcards to a deck and verify entries in SQLite.

- **Basic UI with Markdown**
  - [ ] Create UI elements for:
    - [ ] Deck list (view, create, delete decks).
    - [ ] Flashcard viewer (basic card flipping).
  - [ ] Integrate Markdown rendering for front/back content using a library like `pulldown-cmark`.
  - [ ] Test:
    - [ ] Render Markdown content on flashcards.
    - [ ] Flip a card and verify both sides render correctly.

- **Testing and Debugging**
  - [ ] Test SQLite data persistence after app restarts.
  - [ ] Verify basic CRUD operations and Markdown rendering in the UI.

---

## **Week 2–4: Enhanced MVP**
### **Goal**: Study features, search/filter, and import/export functionality.

### **Week 2: Spaced Repetition and Session Basics**
- **Spaced-Repetition Algorithm**
  - [ ] Integrate a spaced-repetition algorithm.
  - [ ] Use SQLite to track:
    - [ ] Review dates (`next_review_at`).
    - [ ] Success metrics (e.g., `ease_factor`, `interval`).
  - [ ] Test:
    - [ ] Simulate study sessions with spaced-repetition calculations.
    - [ ] Verify updates to `next_review_at` in SQLite.

- **Study Session UI**
  - [ ] Create a study session interface:
    - [ ] Present flashcards based on `next_review_at`.
    - [ ] Allow ranking of answers (easy, hard, incorrect).
  - [ ] Test:
    - [ ] Review flashcards in a session.
    - [ ] Verify user rankings adjust scheduling in SQLite.

- **Search and Filter**
  - [ ] Add search/filter UI:
    - [ ] Filter by tags, difficulty, or deck.
    - [ ] Use SQLite queries for efficient filtering.
  - [ ] Test:
    - [ ] Create multiple decks and flashcards.
    - [ ] Verify filtered results match the query criteria.

- **Debugging and Testing**
  - [ ] Test the full workflow:
    - [ ] Create a deck.
    - [ ] Add flashcards.
    - [ ] Conduct a study session.
    - [ ] Verify spaced-repetition scheduling.

---

### **Week 3: Import/Export and Visual Enhancements**
- **Import/Export Decks**
  - [ ] Implement import/export in JSON and custom `.deck` format.
  - [ ] Write parser and generator for deck files.
  - [ ] Test:
    - [ ] Export a deck, re-import it, and verify data consistency.

- **Dark Mode**
  - [ ] Add a light/dark theme toggle.
  - [ ] Test:
    - [ ] Verify proper rendering in both themes.

- **Markdown Enhancements**
  - [ ] Add support for extended Markdown features:
    - [ ] Headers, bold/italic text, code blocks, and lists.
  - [ ] Test:
    - [ ] Render various Markdown formats on flashcards.

- **Testing and Debugging**
  - [ ] Test deck import/export workflows.
  - [ ] Verify UI consistency in both themes.

---

### **Week 4: Session Summaries and Polish**
- **Session Summaries**
  - [ ] Implement post-session summaries showing:
    - [ ] Cards reviewed.
    - [ ] Accuracy percentages.
    - [ ] Next review schedule.
  - [ ] Test:
    - [ ] Complete study sessions and verify summary data.

- **UI Improvements**
  - [ ] Refine UI for:
    - [ ] Deck list.
    - [ ] Flashcard viewer.
    - [ ] Study session interface.
  - [ ] Test:
    - [ ] Verify smooth navigation and responsiveness.

- **Testing and Debugging**
  - [ ] Conduct full end-to-end testing of the enhanced MVP.
  - [ ] Verify SQLite data integrity for all workflows.

- **Buffer/Additional Testing**
  - [ ] Address any bugs or issues found during testing.

---

## **Month 2: Full MVP**
### **Goal**: Advanced features and app polish.

### **Week 5: Advanced Flashcard Features**
- **Hint System**
  - [ ] Add support for optional hints on flashcards.
  - [ ] Test:
    - [ ] Add hints to flashcards and display them during study sessions.

- **Interconnected Flashcards**
  - [ ] Implement links between related flashcards using SQLite relations.
  - [ ] Test:
    - [ ] Create linked flashcards and navigate between them in the UI.

- **Achievements System**
  - [ ] Add basic achievements:
    - [ ] Study streaks.
    - [ ] Deck completion milestones.
  - [ ] Test:
    - [ ] Trigger achievements under defined conditions.

- **Testing and Debugging**
  - [ ] Test all new features (hints, links, achievements).

---

### **Week 6: Final Features and Polish**
- **Image Support**
  - [ ] Add Markdown image rendering (`![alt text](url)`).
  - [ ] Test:
    - [ ] Create flashcards with images and verify rendering.

- **Progress Tracking and Knowledge Mapping**
  - [ ] Implement analytics using SQLite queries:
    - [ ] Accuracy trends.
    - [ ] Knowledge visualization (e.g., mastery graphs).
  - [ ] Test:
    - [ ] Complete study sessions and verify analytics.

- **Final Debugging and Packaging**
  - [ ] Conduct extensive cross-platform testing.
  - [ ] Fix bugs and optimize performance.
  - [ ] Package the app for distribution using `cargo-bundle` or `cargo-deb`.

- **Final Delivery**
  - [ ] Deliver the completed app with:
    - [ ] Markdown-rendered flashcards.
    - [ ] SQLite-backed storage.
    - [ ] Advanced study features.
    - [ ] Fully functional UI with dark mode and cross-platform compatibility.
