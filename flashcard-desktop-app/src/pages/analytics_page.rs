use leptos::*;

#[component]
pub fn AnalyticsPage() -> impl IntoView {
    view! {
        <main class="container">
            <h1>"Analytics"</h1>
            <p>"Track your study performance and deck progress here."</p>
            <ul>
                <li>"Flashcards reviewed: 50"</li>
                <li>"Accuracy: 85%"</li>
                <li>"Study streak: 10 days"</li>
            </ul>
        </main>
    }
}
