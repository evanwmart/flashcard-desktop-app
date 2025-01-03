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
struct DeckIdArgs {
    deck_id: String,
}

#[component]
pub fn DeckPage(deck_id: String) -> impl IntoView {
    let (deck_name, set_deck_name) = create_signal(String::new());
    let (flashcards, set_flashcards) = create_signal(Vec::<(String, String)>::new());

    // Fetch deck details and flashcards on load
    create_effect({
        let deck_id = deck_id.clone(); // Clone for this closure
        move |_| {
            let deck_id = deck_id.clone(); // Clone for async block
            spawn_local(async move {
                let args = to_value(&DeckIdArgs { deck_id: deck_id.clone() }).unwrap();
                let name_response = invoke("read_deck_name_command", args.clone()).await;
                if let Ok(deck_name_data) = serde_wasm_bindgen::from_value::<String>(name_response) {
                    set_deck_name.set(deck_name_data);
                }

                let flashcards_response = invoke("read_flashcards_command", JsValue::from_bool(true)).await;
                if let Ok(flashcards_data) = serde_wasm_bindgen::from_value::<Vec<(String, String)>>(flashcards_response) {
                    set_flashcards.set(flashcards_data);
                } else {
                    log::error!("Failed to fetch flashcards.");
                }
            });
        }
    });

    view! {
        <div class="deck-page">
            <h1>{deck_name}</h1>
            <div class="flashcards">
                <h2>"Flashcards"</h2>
                <ul>
                    {flashcards.get().iter().map(|(front, back)| {
                        view! {
                            <li>
                                <div class="flashcard">
                                    <p><strong>"Front: "</strong>{front}</p>
                                    <p><strong>"Back: "</strong>{back}</p>
                                </div>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </div>
            <button on:click=move |_| {
                let deck_id = deck_id.clone(); // Clone for async block
                spawn_local(async move {
                    let args = to_value(&DeckIdArgs { deck_id }).unwrap();
                    let response = invoke("delete_deck_command", args).await;
                    if response.is_null() {
                        log::info!("Deck deleted successfully.");
                    } else {
                        log::error!("Failed to delete deck.");
                    }
                });
            }>
                "Delete Deck"
            </button>
        </div>
    }
}
