use leptos::*;
use leptos_router::*;
use crate::components::icons::{
    HomeIcon,
    DeckIcon,
    AnalyticsIcon,
    SettingsIcon
};

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar">
            <ul class="nav-links">
                <li>
                    <A href="/" exact=true class="nav-link" active_class="active">
                        <HomeIcon class=Some("sidebar-icon") />
                    </A>
                </li>
                <li>
                    <A href="/decks" class="nav-link" active_class="active">
                        <DeckIcon class=Some("sidebar-icon") />
                    </A>
                </li>
                <li>
                    <A href="/analytics" class="nav-link" active_class="active">
                        <AnalyticsIcon class=Some("sidebar-icon") />
                    </A>
                </li>
                <li>
                    <A href="/settings" class="nav-link" active_class="active">
                        <SettingsIcon class=Some("sidebar-icon") />
                    </A>
                </li>
            </ul>
        </div>
    }
}
