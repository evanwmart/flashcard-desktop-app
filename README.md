

# **Flashcard Desktop App**

## **Week 1: Initial MVP**
### **Goal**: Core flashcard functionality with Markdown rendering and SQLite setup.

- **Project Setup**
  - [X] Initialize the Rust project with Tauri and Leptos.
  - [ ] Configure Tauri for cross-platform builds.
    - [ ] Windows
    - [X] Linux
    - [ ] MacOS (To be addressed later)
  - [ ] Set up frontend-reactive components in Leptos.

- **SQLite Integration**
  - [ ] Use `sqlx` for async SQLite integration.
  - [ ] Define database schema for decks and flashcards:
    - Tables:
      - `decks` (`id`, `name`, `created_at`).
      - `flashcards` (`id`, `deck_id`, `front_md`, `back_md`, `created_at`).
  - [ ] Write migration scripts and validate schema.

- **Flashcard CRUD Operations**
  - [ ] Backend logic for creating, reading, updating, and deleting decks and flashcards.
  - [ ] Connect frontend (Leptos) to SQLite backend using Tauri commands.
  - [ ] Test:
    - [ ] Create decks and flashcards.
    - [ ] Validate data persistence in SQLite.

- **Basic UI with Markdown**
  - [ ] Develop UI components in Leptos for:
    - [ ] Deck list (create, view, delete).
    - [ ] Flashcard viewer with card-flipping.
      - [ ] Display Markdown content for front and back using a library like `comrak`.
      - [ ] Add flip animations for flashcards.
  - [ ] Test:
    - [ ] Render Markdown on both sides of a card.

- **Testing and Debugging**
  - [ ] Verify data persistence after app restarts.
  - [ ] Debug CRUD workflows and UI rendering.

---

## **Week 2–3: Study Features and Spaced Repetition**
### **Goal**: Spaced repetition, study sessions, and search/filter functionality.

### **Week 2: Spaced Repetition**
- **Spaced Repetition Algorithm**
  - [ ] Integrate a spaced-repetition algorithm (e.g., SM-2).
  - [ ] Use SQLite to store and update:
    - `next_review_at`
    - `ease_factor`, `interval`, and success metrics.
  - [ ] Test:
    - Simulate flashcard reviews and verify updates to `next_review_at`.

- **Study Session UI**
  - [ ] Create a study session interface:
    - [ ] Display flashcards based on `next_review_at`.
    - [ ] Allow rating answers (easy, hard, incorrect).
  - [ ] Test:
    - Verify user interactions update scheduling in the backend.

### **Week 3: Search, Filter, and Tagging**
- **Search/Filter UI**
  - [ ] Add UI filters:
    - By tags, difficulty, or deck.
  - [ ] Optimize search using SQLite queries.
  - [ ] Test:
    - Validate filtered flashcards and decks.

- **Tagging System**
  - [ ] Add support for tagging flashcards.
  - [ ] Test:
    - Ensure tags are stored and queried properly.

---

## **Week 4: Import/Export and UI Enhancements**
### **Goal**: Smooth workflows, JSON import/export, and improved UI.

- **Import/Export Decks**
  - [ ] Support JSON and custom `.deck` format.
  - [ ] Test:
    - Export and re-import decks for consistency.

- **Dark Mode**
  - [ ] Add theme toggle for light/dark mode.
  - [ ] Test:
    - Verify UI elements render correctly in both themes.

- **Markdown Enhancements**
  - [ ] Support extended Markdown features:
    - Headers, bold/italic, lists, and code blocks.
  - [ ] Test:
    - Validate Markdown rendering for various content types.

- **UI Polish**
  - [ ] Refine animations for deck/flashcard interactions.
  - [ ] Improve responsiveness and accessibility.

---

## **Month 2: Full MVP and Polish**
### **Goal**: Session summaries, analytics, and advanced features.

### **Week 5: Advanced Features**
- **Session Summaries**
  - [ ] Post-study summaries showing:
    - Cards reviewed, accuracy, and next reviews.
  - [ ] Test:
    - Verify session metrics.

- **Hints and Interconnected Flashcards**
  - [ ] Add optional hints to flashcards.
  - [ ] Implement linking between flashcards for related content.
  - [ ] Test:
    - Ensure seamless navigation between linked flashcards.

- **Achievements**
  - [ ] Introduce achievements:
    - Study streaks and deck completions.
  - [ ] Test:
    - Validate triggers for achievements.

### **Week 6: Final Features and Packaging**
- **Image Support**
  - [ ] Render images in Markdown (`![alt](url)`).
  - [ ] Test:
    - Verify images display properly.

- **Progress Analytics**
  - [ ] Track study performance:
    - Accuracy trends and card mastery stats.
  - [ ] Add basic visualization (e.g., graphs).

- **Cross-Platform Packaging**
  - [ ] Package app with Tauri for:
    - Windows, Linux, and MacOS.
  - [ ] Test:
    - Ensure full functionality across platforms.

- **Final Debugging and Polish**
  - [ ] Conduct end-to-end testing.
  - [ ] Address performance optimizations and bugs.

- **Delivery**
  - [ ] Finalize app with Markdown-rendered flashcards, SQLite persistence, and polished UI.

