use iced::{
    widget::{Column, Button, Text, Scrollable, Container},
    Element, Length,
};
use crate::utils::markdown::render_markdown_to_html;

#[derive(Debug, Clone)]
pub enum FlashcardMessage {
    FlipCard,
}

pub struct FlashcardViewer {
    front: String,
    back: String,
    show_front: bool,
}

impl FlashcardViewer {
    pub fn new(front: String, back: String) -> Self {
        Self {
            front,
            back,
            show_front: true,
        }
    }

    pub fn update(&mut self, _message: FlashcardMessage) {
        self.show_front = !self.show_front;
    }

    pub fn view(&self) -> Element<FlashcardMessage> {
        let content = if self.show_front {
            render_markdown_to_html(&self.front)
        } else {
            render_markdown_to_html(&self.back)
        };

        let card_content = Container::new(Text::new(content).size(18))
            .padding(20)
            .width(Length::Fill)
            .center_x(Length::Shrink); // Pass Length::Shrink to center_x

        Column::new()
            .spacing(20)
            .align_x(iced::alignment::Horizontal::Center) // Center horizontally
            .push(Text::new("Flashcard Viewer").size(30))
            .push(Scrollable::new(card_content).height(Length::Fill))
            .push(Button::new(Text::new("Flip Card")).on_press(FlashcardMessage::FlipCard))
            .padding(20)
            .into()
    }
}
