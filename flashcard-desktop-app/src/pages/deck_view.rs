// deck_view.rs
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn DeckView() -> impl IntoView {
    let flashcards = vec!["Card 1", "Card 2"];
    let navigate = use_navigate();

    view! {
        <main class="container">
            <h1>"Deck: Sample Deck"</h1>
            {
                let navigate = navigate.clone();
                view! {
                    <button on:click=move |_| navigate("/editor/new", Default::default())>
                        "Add Flashcard"
                    </button>
                }
            }
            {
                let navigate = navigate.clone();
                view! {
                    <button on:click=move |_| navigate("/study/1", Default::default())>
                        "Start Study Session"
                    </button>
                }
            }

            <h2>"Flashcards"</h2>
            <ul>
                {flashcards.into_iter().map(|card| view! {
                    <li>
                        <span>{card}</span>
                        <button>"Edit"</button>
                        <button>"Delete"</button>
                    </li>
                }).collect_view()}
            </ul>
        </main>
    }
}