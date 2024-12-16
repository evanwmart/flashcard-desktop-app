use iced::widget::{Column, Row, Text, Button, TextInput, Scrollable};
use iced::{Element, Length, Task};
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
                if !self.new_deck_name.is_empty() {
                    let conn = initialize_database(false).unwrap();
                    create_deck(&conn, &self.new_deck_name, "Default Pack", "2024-12-15").unwrap();
                    self.new_deck_name.clear();
                    return Task::perform(Self::fetch_decks(), Message::DecksLoaded);
                }
                Task::none()
            }
            Message::DeleteDeck(deck_id) => {
                let conn = initialize_database(false).unwrap();
                delete_deck(&conn, deck_id).unwrap();
                return Task::perform(Self::fetch_decks(), Message::DecksLoaded);
            }
            Message::DecksLoaded(decks) => {
                self.decks = decks;
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
                    .push(Text::new(&deck.1))
                    .push(Button::new(Text::new("Delete"))
                          .on_press(Message::DeleteDeck(deck.0))),
            )
        });

        let input_row = Row::new()
            .spacing(10)
            .push(
                TextInput::new("Enter deck name", &self.new_deck_name)
                    .on_input(Message::DeckNameChanged)
                    .padding(10)
                    .width(Length::Fill),
            )
            .push(Button::new(Text::new("Create Deck"))
                  .on_press(Message::CreateDeck));

        Column::new()
            .spacing(20)
            .push(Text::new("Decks").size(30))
            .push(Scrollable::new(deck_list).height(Length::Fill))
            .push(input_row)
            .padding(20)
            .into()
    }

    fn fetch_decks() -> impl std::future::Future<Output = Vec<(i64, String, String, String)>> {
        async {
            let conn = initialize_database(false).unwrap();
            read_decks(&conn).unwrap_or_default()
        }
    }
}
