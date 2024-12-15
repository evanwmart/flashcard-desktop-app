use iced::widget::{button, text};
use iced::{Element, Length};

pub fn flashcard_view(counter: &u64) -> Element<()> {
    button(text(format!("Flashcard Count: {}", counter)))
        .width(Length::Shrink)
        .into()
}
