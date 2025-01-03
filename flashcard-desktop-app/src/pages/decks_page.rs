use leptos::*;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "invoke"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct CreateDeckArgs {
    name: String,
    created_date: String,
    style: Option<String>,
}

#[component]
pub fn DecksPage() -> impl IntoView {
    let (decks, set_decks) = create_signal(Vec::<(i32, String, i32, String)>::new());

    // Fetch decks from Tauri on load
    create_effect(move |_| {
        spawn_local(async move {
            let response = invoke("read_decks_command", JsValue::NULL).await;
            if let Ok(decks_data) = serde_wasm_bindgen::from_value::<Vec<(i32, String, i32, String)>>(response) {
                set_decks.set(decks_data);
            } else {
                log::error!("Failed to fetch decks from Tauri.");
            }
        });
    });

    view! {
        <div class="decks-page">
            <h1>"Decks"</h1>
            <table class="decks-table">
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"# Cards Today"</th>
                        <th>"Last Study"</th>
                    </tr>
                </thead>
                <tbody>
                    {decks.get().iter().map(|(id, name, cards_to_study, last_study_date)| {
                        view! {
                            <tr>
                                <td>
                                    <a href=format!("/deck/{}", id)>
                                        {name}
                                    </a>
                                </td>
                                <td>{cards_to_study.to_string()}</td>
                                <td>{last_study_date}</td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
            <button on:click=move |_| {
                spawn_local(async move {
                    let args = CreateDeckArgs {
                        name: "New Deck".to_string(),
                        created_date: "2025-01-01".to_string(),
                        style: None,
                    };
                    let args = to_value(&args).unwrap();
                    let response = invoke("create_deck_command", args).await;
                    if response.is_null() {
                        log::info!("Deck added successfully.");
                    } else {
                        log::error!("Failed to add new deck.");
                    }
                });
            }>
                "Add New Deck"
            </button>
        </div>
    }
}
