use leptos::*;
use leptos_router::*;
use crate::components::icons::{EditIcon, TrashIcon, DotsIcon};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn DecksPage() -> impl IntoView {
    // Reactive signal to store decks
    let decks = create_resource(
        || (), 
        |_| async {
            // Fetch decks from the backend using the Tauri command
            let args = JsValue::from_serde(&Value::Null).unwrap();
            let result = invoke("read_decks_command", args).await;
            result.into_serde::<Vec<(i32, String, usize, usize)>>()
                .unwrap_or_else(|_| vec![]) // Handle errors gracefully
        }
    );

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
                    // Use the resource signal to render the decks
                    {move || decks().map(|decks| {
                        decks.into_iter().map(|(id, name, cards_to_review, progress)| {
                            let navigate_to_deck = {
                                let navigate = navigate.clone();
                                move |_| navigate(&format!("/deck/{}", id), Default::default())
                            };

                            view! {
                                <tr class="clickable-row" on:click=navigate_to_deck>
                                    <td>{name}</td>
                                    <td>{cards_to_review}</td>
                                    <td>"Today"</td>
                                    <td>
                                        <div class="progress-container">
                                            {format!("{}%", progress)}
                                            <div
                                                class="progress-circle"
                                                style={format!("--progress: {};", progress)}
                                            ></div>
                                        </div>
                                    </td>
                                    <td></td>
                                </tr>
                            }
                        }).collect_view()
                    })}
                </tbody>
            </table>

            <button class="create-button" on:click=move |_| {
                spawn_local(async {
                    let args = JsValue::from_serde(&serde_json::json!({
                        "name": "New Deck",
                        "created_date": "2025-01-01",
                        "style": "default"
                    })).unwrap();
                    
                    invoke("create_deck_command", args)
                        .await
                        .unwrap_or_else(|e| {
                            log::error!("Failed to create deck: {}", e);
                            JsValue::NULL
                        });
                });
            }>
                "+"
            </button>
        </main>
    }
}