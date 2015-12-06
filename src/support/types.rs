use std::fmt;

#[derive(Debug, Clone)]
pub struct Population {
    pub decks : Vec<Deck>
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards : Vec<Card>,
    pub fitness : f32,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub name : String,
    pub cardtype : String,
    pub mana_cost : String,
    pub colors : Vec<String>,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, type: {}, manaCost: {}", self.name, self.cardtype, self.mana_cost)
    }
}
