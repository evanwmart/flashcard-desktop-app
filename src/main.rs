mod db;        // Database module
mod models;    // Models module
mod ui;        // UI module

use db::initialize_database;
use iced::widget::{button, text};
use iced::{Element, Length, Result};

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

pub fn main() -> Result {
    initialize_database().expect("Failed to initialize database");
    iced::run("Flashcard Desktop App", update, view)
}

fn update(counter: &mut u64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
    }
}

fn view(counter: &u64) -> Element<Message> {
    button(text(format!("Count: {}", counter)))
        .on_press(Message::Increment)
        .width(Length::Shrink)
        .into()
}
