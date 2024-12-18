// app.rs
use leptos::*;
use leptos_router::*;
use leptos_router::use_navigate;

use crate::pages::{
    analytics::AnalyticsPage,
    deck_view::DeckView,
    editor::EditorPage,
    import::ImportPage,
    settings::SettingsPage,
    study::StudyPage,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <a href="/">"Home"</a>
                <a href="/import">"Import/Export"</a>
                <a href="/settings">"Settings"</a>
                <a href="/analytics">"Analytics"</a>
            </nav>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/deck/:id" view=DeckView/>
                <Route path="/editor/:id" view=EditorPage/>
                <Route path="/study/:id" view=StudyPage/>
                <Route path="/import" view=ImportPage/>
                <Route path="/settings" view=SettingsPage/>
                <Route path="/analytics" view=AnalyticsPage/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let decks = vec!["Deck 1", "Deck 2"];
    let navigate = use_navigate();

    view! {
        <main class="container">
            <h1>"Flashcard App Dashboard"</h1>
            {
                let navigate = navigate.clone();
                view! {
                    <button on:click=move |_| navigate("/editor/new", Default::default())>
                        "Create New Deck"
                    </button>
                }
            }
            {
                let navigate = navigate.clone();
                view! {
                    <button on:click=move |_| navigate("/import", Default::default())>
                        "Import Deck"
                    </button>
                }
            }

            <h2>"Your Decks"</h2>
            <ul>
                {decks.into_iter().map({
                    let navigate = navigate.clone();
                    move |deck| {
                        let deck_name = deck.to_string();
                        let navigate = navigate.clone();
                        view! {
                            <li>
                                <span>{deck_name.clone()}</span>
                                <button on:click=move |_| navigate(&format!("/deck/{}", deck_name), Default::default())>
                                    "View Deck"
                                </button>
                            </li>
                        }
                    }
                }).collect_view()}
            </ul>
        </main>
    }
}