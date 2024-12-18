use leptos::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <main class="container">
            <h1>"Settings"</h1>
            <label>
                <input type="checkbox"/> "Dark Mode"
            </label>
            <button>"Reset App Data"</button>
        </main>
    }
}
