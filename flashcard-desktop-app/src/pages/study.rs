use leptos::*;
use std::rc::Rc;

#[component]
pub fn StudyPage() -> impl IntoView {
    let flashcards = Rc::new(vec![
        ("Front 1", "Back 1"),
        ("Front 2", "Back 2"),
    ]);
    let (index, set_index) = create_signal(0);

    let flashcards_ref = Rc::clone(&flashcards);
    let next_card = move |_| {
        set_index.update(|i| *i = (*i + 1) % flashcards_ref.len());
    };

    view! {
        <main class="container">
            <h1>"Study Session"</h1>
            <div>
                <h2>"Front:"</h2>
                <p>{flashcards[index.get()].0}</p>
                <button on:click=next_card>"Next Card"</button>
            </div>
        </main>
    }
}
