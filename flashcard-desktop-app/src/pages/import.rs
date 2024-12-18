use leptos::*;

#[component]
pub fn ImportPage() -> impl IntoView {
    view! {
        <main class="container">
            <h1>"Import/Export Decks"</h1>
            <h2>"Import"</h2>
            <input type="file" accept=".json, .deck"/>
            <button>"Import"</button>

            <h2>"Export"</h2>
            <button>"Export as JSON"</button>
            <button>"Export as .deck"</button>
        </main>
    }
}
