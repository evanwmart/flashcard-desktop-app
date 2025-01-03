use leptos::*;
use leptos_router::*;
use crate::components::icons::{EditIcon, TrashIcon};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn DeckPage() -> impl IntoView {
    let params = use_params_map();
    let deck_id = params.with(|p| p.get("deck_id").cloned().unwrap_or_else(|| "Unknown Deck".to_string()));

    // Reactive signal for flashcards
    let flashcards = create_resource(
        move || deck_id.clone(),
        |deck_id| async move {
            let args = JsValue::from_serde(&serde_json::json!({
                "deck_id": deck_id.parse::<i32>().unwrap_or(0)
            })).unwrap();
            
            let result = invoke("read_flashcards_command", args).await;
            result.into_serde::<Vec<(i32, String, String)>>()
                .unwrap_or_else(|_| vec![]) // Handle errors gracefully
        }
    );

    view! {
        <main class="container">
            <div class="deck-top-section">
                <div class="deck-overview">
                    <h1 class="deck-title">{deck_id}</h1>
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
                    <button class="add-button" 
                        on:click=move |_| {
                            let deck_id = deck_id.clone();
                            spawn_local(async move {
                                let args = JsValue::from_serde(&serde_json::json!({
                                    "deck_id": deck_id,
                                    "html_front": "New Front",
                                    "html_back": "New Back"
                                })).unwrap();
                                
                                invoke("create_flashcard_command", args)
                                    .await
                                    .unwrap_or_else(|e| {
                                        log::error!("Failed to create flashcard: {}", e);
                                        JsValue::NULL
                                    });
                            });
                        }
                    >
                        "Add Flashcard"
                    </button>
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
                            {move || {
                                flashcards.read().map(|cards| {
                                    cards.into_iter().map(|(id, front, back)| {
                                        view! {
                                            <tr>
                                                <td>{front}</td>
                                                <td>{back}</td>
                                                <td class="actions-cell">
                                                    <button class="action-btn" 
                                                        on:click=move |_| {
                                                            let id = id;
                                                            spawn_local(async move {
                                                                let args = JsValue::from_serde(&serde_json::json!({
                                                                    "id": id,
                                                                    "html_front": "Updated Front",
                                                                    "html_back": "Updated Back"
                                                                })).unwrap();
                                                                
                                                                invoke("update_flashcard_command", args)
                                                                    .await
                                                                    .unwrap_or_else(|e| {
                                                                        log::error!("Failed to update flashcard: {}", e);
                                                                        JsValue::NULL
                                                                    });
                                                            });
                                                        }
                                                    >
                                                        <EditIcon class=Some("icon") />
                                                    </button>
                                                    <button class="action-btn" 
                                                        on:click=move |_| {
                                                            let id = id;
                                                            spawn_local(async move {
                                                                let args = JsValue::from_serde(&serde_json::json!({
                                                                    "id": id
                                                                })).unwrap();
                                                                
                                                                invoke("delete_flashcard_command", args)
                                                                    .await
                                                                    .unwrap_or_else(|e| {
                                                                        log::error!("Failed to delete flashcard: {}", e);
                                                                        JsValue::NULL
                                                                    });
                                                            });
                                                        }
                                                    >
                                                        <TrashIcon class=Some("icon") />
                                                    </button>
                                                </td>
                                            </tr>
                                        }
                                    }).collect_view()
                                })
                            }}
                        </tbody>
                    </table>
                </div>
            </div>
        </main>
    }
}