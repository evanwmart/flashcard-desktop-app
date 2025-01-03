// app.rs
use leptos::*;
use leptos_router::*;

use crate::pages::{
    analytics_page::AnalyticsPage,
    deck_page::DeckPage,
    decks_page::DecksPage,
    settings_page::SettingsPage,
    study_page::StudyPage,
};

use crate::components::sidebar::Sidebar;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app-layout">
                <Sidebar />
                <div class="main-content">
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/decks" view=DecksPage/>
                        <Route path="/deck/:deck_id" view=DeckPageWrapper/>
                        <Route path="/study/:deck_id" view=StudyPage/>
                        <Route path="/analytics" view=AnalyticsPage/>
                        <Route path="/settings" view=SettingsPage/>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}

#[component]
fn DeckPageWrapper() -> impl IntoView {
    let params = use_params_map(); // Get the route parameters map
    let deck_id = params.with(|map| map.get("deck_id").cloned().unwrap_or_else(|| "Unknown".to_string()));

    view! {
        <DeckPage deck_id=deck_id/>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="container">
            <h1>"Welcome to the Flashcard App"</h1>
            <p>"Your personal flashcard manager to enhance learning."</p>
            <ul>
                <li>"View your decks"</li>
                <li>"Track analytics"</li>
                <li>"Start a study session"</li>
            </ul>
        </main>
    }
}
