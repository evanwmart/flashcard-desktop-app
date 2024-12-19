use leptos::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <main class="container">
            <h1>"Settings"</h1>
            <p>"Customize your preferences here."</p>
            <ul>
                <li>
                    <label for="dark-mode">"Dark Mode:"</label>
                    <input type="checkbox" id="dark-mode" />
                </li>
                <li>
                    <label for="notifications">"Enable Notifications:"</label>
                    <input type="checkbox" id="notifications" />
                </li>
            </ul>
        </main>
    }
}
