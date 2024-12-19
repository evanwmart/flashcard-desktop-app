use leptos::*;
use leptos_router::*;
use crate::components::icons::{ EditIcon, TrashIcon, DotsIcon };

#[component]
pub fn DecksPage() -> impl IntoView {
    let decks = vec![
        ("Biology", 12, "20 min ago", 85),
        ("Math", 8, "Yesterday", 65),
        ("History", 5, "2 days ago", 90)
    ];

    let navigate = use_navigate();

    view! {
        <main class="container">
            <div class="decks-header">
                <h1>"Your Decks"</h1>
            </div>

            <table class="decks-table">
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Cards to Review"</th>
                        <th>"Last Reviewed"</th>
                        <th>"Progress"</th>
                        <th>""</th> 
                    </tr>
                </thead>
                <tbody>
                    {decks.into_iter().map(|(name, cards, last_reviewed, progress)| {
                        let navigate_to_deck = {
                            let navigate = navigate.clone();
                            move |_| navigate(&format!("/deck/{}", name), Default::default())
                        };

                        view! {
                            <tr class="clickable-row" on:click=navigate_to_deck>
                                <td>
                                    <div class="toggle-container">
                                        <span>{name}</span>
                                    </div>
                                </td>
                                <td>{cards}</td>
                                <td>{last_reviewed}</td>
                                <td>
                                    <div class="progress-container">
                                            {format!("{}%", progress)}
                                        <div
                                            class="progress-circle"
                                            style={format!("--progress: {};", progress)}
                                        >
                                        </div>
                                    </div>
                                </td>
                                <td></td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>

            <button class="create-button" on:click=move |_| { /* Add your action */ }>
            "+"
        </button>
        
        </main>
    }
}
