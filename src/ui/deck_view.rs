use iced::{
    widget::{Column, Row, Text, Button, TextInput, Scrollable, Container, Space},
    Element, Length, Task,
};
use crate::db::{initialize_database, create_deck, delete_deck, read_decks};

#[derive(Debug, Clone)]
pub enum Message {
    DeckNameChanged(String),
    CreateDeck,
    DeleteDeck(i64),
    DecksLoaded(Vec<(i64, String, String, String)>),
    None,
}

pub struct DeckView {
    decks: Vec<(i64, String, String, String)>,
    new_deck_name: String,
}
impl DeckView {
    pub fn new() -> Self {
        Self {
            decks: Vec::new(),
            new_deck_name: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DeckNameChanged(name) => {
                self.new_deck_name = name;
                Task::none()
            }
            Message::CreateDeck => {
                // User manually creates a deck
                if !self.new_deck_name.is_empty() {
                    let deck_name = self.new_deck_name.clone();
                    self.new_deck_name.clear();
                    Task::perform(Self::create_and_reload(deck_name), Message::DecksLoaded)
                } else {
                    Task::none()
                }
            }
            Message::DeleteDeck(deck_id) => {
                println!("Attempting to delete deck with ID: {}", deck_id);
                Task::perform(Self::delete_and_reload(deck_id), Message::DecksLoaded)
            }
            Message::DecksLoaded(decks) => {
                self.decks = decks;
                println!("Updated Decks: {:?}", self.decks);
                Task::none()
            }
            Message::None => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let deck_list = self.decks.iter().fold(Column::new().spacing(10), |column, deck| {
            column.push(
                Row::new()
                    .spacing(10)
                    .align_y(iced::alignment::Vertical::Center)
                    .push(Text::new(&deck.1).size(20))
                    .push(Button::new(Text::new("Delete")).on_press(Message::DeleteDeck(deck.0))),
            )
        });

        let input_row = Row::new()
            .spacing(10)
            .align_y(iced::alignment::Vertical::Center)
            .push(
                TextInput::new("Enter deck name", &self.new_deck_name)
                    .on_input(Message::DeckNameChanged)
                    .padding(10)
                    .width(Length::Fill),
            )
            .push(
                Button::new(Text::new("Create Deck").size(16))
                    .on_press(Message::CreateDeck)
                    .padding(10),
            );

        Container::new(
            Column::new()
                .spacing(20)
                .align_x(iced::alignment::Horizontal::Center)
                .push(Text::new("Decks").size(30))
                .push(Space::with_height(Length::Fixed(10.0)))
                .push(Scrollable::new(deck_list).height(Length::Fill))
                .push(input_row),
        )
        .padding(20)
        .center_x(Length::Shrink)
        .into()
    }

    /// Fetch decks when the app starts (only fetch, no creation)
    pub fn fetch_decks_on_start() -> Task<Message> {
        Task::perform(Self::fetch_decks_and_reload(), Message::DecksLoaded)
    }

    async fn fetch_decks_and_reload() -> Vec<(i64, String, String, String)> {
        let conn = initialize_database(false).unwrap();
        let decks = read_decks(&conn).unwrap_or_default();
        println!("Fetched Decks: {:?}", decks);
        decks
    }

    async fn create_and_reload(name: String) -> Vec<(i64, String, String, String)> {
        let conn = initialize_database(false).unwrap();
        // Explicitly insert a new deck only when triggered by the user
        create_deck(&conn, &name, "Default Pack", "2024-12-15").unwrap();
        Self::fetch_decks_and_reload().await
    }

    async fn delete_and_reload(deck_id: i64) -> Vec<(i64, String, String, String)> {
        let conn = initialize_database(false).unwrap();
        let rows_affected = delete_deck(&conn, deck_id).unwrap_or(0);
    
        if rows_affected > 0 {
            println!("Successfully deleted Deck ID: {}", deck_id);
        } else {
            println!("Deck ID {} not found or already deleted.", deck_id);
        }
    
        Self::fetch_decks_and_reload().await
    }
}
