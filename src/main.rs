use flashcard_desktop_app::{
    db::initialize_database,
    ui::deck_view::{DeckView, Message},
};
use iced::{self, Theme, widget::{Column, Text}, Element, Task};

struct App {
    deck_view: DeckView,
}

fn main() -> iced::Result {
    initialize_database(false).expect("Failed to initialize database");

    iced::application("Flashcard Desktop App", update, view)
        .theme(|_| Theme::default())
        .run_with(
            || {
                let state = App {
                    deck_view: DeckView::new(),
                };
                (state, Task::none())
            }
        )
}

fn update(state: &mut App, message: Message) {
    let _task = state.deck_view.update(message);
}

fn view(state: &App) -> Element<Message> {
    Column::new()
        .push(Text::new("Decks:"))
        .push(state.deck_view.view())
        .into()
}
