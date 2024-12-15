pub struct Deck {
    pub id: i64,
    pub name: String,
    pub pack: String,
    pub created_at: String,
}

impl Deck {
    pub fn new(id: i64, name: &str, pack: &str, created_at: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            pack: pack.to_string(),
            created_at: created_at.to_string(),
        }
    }
}
