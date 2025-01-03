# **Flashcard Desktop App**

A Tauri + Leptos application for creating and reviewing flashcards, storing data in SQLite via **tokio-rusqlite**.

## **Week 1: Initial MVP**
### **Goal**: Core flashcard functionality with HTML rendering and SQLite setup.

- **Project Setup**
  - [X] Initialize the Rust project with Tauri and Leptos.
  - [ ] Configure Tauri for cross-platform builds.
    - [ ] Windows
    - [X] Linux
    - [ ] MacOS (To be addressed later)
  - [X] Set up frontend-reactive components in Leptos.

- **SQLite Integration**
  - [X] Use **tokio-rusqlite** for async SQLite integration.
  - [X] Define database schema for decks and flashcards:
    - Tables:
      - `buckets`
      - `decks`
      - `flashcards`
      - `media`
      - `sessions`
      - `settings`
      - `topics`
  - [ ] Validate schema correctness.

- **CRUD Operations**
  - [X] Backend logic for CRUD operations using Tauri commands.
  - [ ] Connect frontend (Leptos) to SQLite backend for actual usage.
  - [ ] Verify Database persistence across app restarts.

- **Basic UI with HTML**
  - [ ] Develop UI components in Leptos for:
    - [ ] Deck list (create, view, delete).
    - [ ] Flashcard viewer with card-flipping.
      - [ ] Display **HTML** content for front and back.
      - [ ] Add flip animations for flashcards.
  - [ ] Test:
    - [ ] Ensure HTML rendering properly for both sides of a card.

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
    - `ease_factor`, `interval`, and performance metrics.
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
  - [ ] Add UI filters (tags, difficulty, or deck).
  - [ ] Optimize search using SQLite queries.
  - [ ] Test:
    - Validate filtering or searching for flashcards and decks.

- **Tagging System**
  - [ ] Add support for tagging flashcards.
  - [ ] Test:
    - Ensure tags are stored and queried properly.

---

## **Week 4: Import/Export and UI Enhancements**
### **Goal**: Smooth workflows, JSON import/export, and improved UI.

- **Import/Export Decks**
  - [ ] Support JSON or a custom `.deck` format.
  - [ ] Test:
    - Export decks and re-import them for consistency.

- **Dark Mode**
  - [ ] Add a theme toggle for light/dark mode.
  - [ ] Test:
    - Verify UI elements render correctly in both themes.

- **HTML/Rendering Enhancements**
  - [ ] Support extended HTML features:
    - Lists, images, code blocks, etc.
  - [ ] Test:
    - Validate HTML rendering with various content types (not using Markdown).

- **UI Polish**
  - [ ] Refine animations for deck/flashcard interactions.
  - [ ] Improve responsiveness and accessibility.

---

## **Month 2: Full MVP and Polish**
### **Goal**: Session summaries, analytics, and advanced features.

### **Week 5: Advanced Features**
- **Session Summaries**
  - [ ] Post-study summaries showing:
    - Cards reviewed, accuracy, next reviews, etc.
  - [ ] Test:
    - Verify session metrics stored and displayed accurately.

- **Hints and Interconnected Flashcards**
  - [ ] Optional hints for flashcards.
  - [ ] Linking between flashcards for related content.
  - [ ] Test:
    - Ensure smooth navigation between linked flashcards.

- **Achievements**
  - [ ] Introduce achievements (study streaks, deck completion).
  - [ ] Test:
    - Validate triggers for achievements.

### **Week 6: Final Features and Packaging**
- **Media Support**
  - [ ] Render images, audio, or other media in the flashcards.
  - [ ] Test:
    - Confirm media display for front/back content.

- **Progress Analytics**
  - [ ] Track study performance:
    - Accuracy over time, card mastery trends.
  - [ ] Add basic visualization (graphs or stats).

- **Cross-Platform Packaging**
  - [ ] Package the app for Windows, Linux, and MacOS via Tauri.
  - [ ] Test:
    - Ensure all features work consistently across platforms.

- **Final Debugging and Polish**
  - [ ] Conduct thorough end-to-end testing.
  - [ ] Address performance optimizations and leftover bugs.

- **Delivery**
  - [ ] Finalize the app with HTML-rendered flashcards, SQLite persistence, and a polished UI.

---
