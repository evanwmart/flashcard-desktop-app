use iced::widget::{Column, Text};
use iced::Element;
use crate::utils::markdown::render_markdown_to_html;

/// Displays a flashcard with Markdown-rendered content.
///
/// # Arguments
///
/// * `front` - Markdown content for the front of the card.
/// * `back` - Markdown content for the back of the card.
///
/// # Returns
///
/// An Iced `Element` displaying the flashcard.
pub fn flashcard_view<'a>(front: &'a str, back: &'a str, show_front: bool) -> Element<'a, ()> {
    let content = if show_front { front } else { back };
    let rendered_html = render_markdown_to_html(content);

    Column::new()
        .push(Text::new(rendered_html)) // Replace with Iced widgets that can style HTML-like content
        .into()
}

