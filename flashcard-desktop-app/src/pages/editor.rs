use leptos::*;
use leptos_router::use_navigate;


#[component]
pub fn EditorPage() -> impl IntoView {
    let (_front, set_front) = create_signal(String::new());
    let (_back, set_back) = create_signal(String::new());

    let navigate = use_navigate();

    view! {
        <main class="container">
            <h1>"Flashcard Editor"</h1>
            <label>"Front Content (Markdown)"</label>
            <textarea on:input=move |e| set_front.set(event_target_value(&e))></textarea>

            <label>"Back Content (Markdown)"</label>
            <textarea on:input=move |e| set_back.set(event_target_value(&e))></textarea>

            <button on:click=move |_| { /* Save logic */ }>"Save Flashcard"</button>
            <button on:click=move |_| navigate("/", Default::default())>"Cancel"</button>
        </main>
    }
}
