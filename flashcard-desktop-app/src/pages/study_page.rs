use leptos::*;

#[component]
pub fn StudyPage() -> impl IntoView {
    // Destructure the signal tuple into read and write parts
    let (flashcards, _set_flashcards) = create_signal(vec![
        ("Front 1", "Back 1"),
        ("Front 2", "Back 2"),
    ]);
    let (index, set_index) = create_signal(0);
    
    let next_card = move |_| {
        set_index.update(|i| *i = (*i + 1) % flashcards.get().len());
    };
    
    view! {
        <main class="container">
            <h1>"Study Session"</h1>
            <div>
                <h2>"Question:"</h2>
                <p>{move || flashcards.get()[index.get()].0}</p>
                <h2>"Answer:"</h2>
                <p>{move || flashcards.get()[index.get()].1}</p>
                <button on:click=next_card>"Next Card"</button>
            </div>
        </main>
    }
}