use leptos::*;
use leptos_router::*;
use crate::components::icons::{EditIcon, TrashIcon};

#[component]
pub fn DeckPage() -> impl IntoView {
    let params = use_params_map();
    let deck_id = params.with(|p| p.get("deck_id").cloned());

    let flashcards = vec![
        ("What is Rust?", "A systems programming language."),
        ("Ownership?", "A set of rules to manage memory."),
        ("Lifetime?", "The scope for which a reference is valid."),
    ];

    view! {
        <main class="container">

            <div class="deck-top-section">
                <div class="deck-overview">
                    <h1 class="deck-title">
                        {deck_id.unwrap_or_else(|| "Unknown Deck".to_string())}
                    </h1>
                    <button class="study-button">"Start Study Session"</button>
                    <p class="last-reviewed">"Last Reviewed: Yesterday"</p>
                    <div class="progress-container">
                        <div class="progress-circle" style="--progress: 75;"></div>
                        <span class="progress-text">"75% Complete"</span>
                    </div>
                </div>

                <div class="deck-mindmap">
                    <h2>"Knowledge Map"</h2>
                    <div class="mindmap-placeholder">
                        <p>"Mind map visualization goes here..."</p>
                    </div>
                </div>
            </div>

            <div class="deck-bottom-section">
                <div class="table-header">
                    <h2>"Flashcards"</h2>
                    <button class="add-button">"Add Flashcard"</button>
                </div>
                <div class="table-container">
                    <table class="flashcards-table">
                        <thead>
                            <tr>
                                <th>"Front"</th>
                                <th>"Back"</th>
                                <th>""</th>
                            </tr>
                        </thead>
                        <tbody>
                            {flashcards.into_iter().map(|(front, back)| {
                                view! {
                                    <tr>
                                        <td>{front}</td>
                                        <td>{back}</td>
                                        <td class="actions-cell">
                                            <button class="action-btn">
                                                <EditIcon class=Some("icon") />
                                            </button>
                                            <button class="action-btn">
                                                <TrashIcon class=Some("icon") />
                                            </button>
                                        </td>
                                    </tr>
                                }
                            }).collect_view()}
                        </tbody>
                    </table>
                </div>
            </div>

        </main>
    }
}
