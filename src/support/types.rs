use std::fmt;

pub struct Population {
    pub decks : Vec<Deck>
}

pub struct Deck {
    pub cards : Vec<Card>,
    pub fitness : f32,
}

pub struct Card {
    pub name : String,
    pub cardtype : String,
    pub cmc : i32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name:{}, type: {}, cmc {}", self.name, self.cardtype, self.cmc)
    }
}

//Static declarations for testing. This should not be in the final release
// pub static Mountain: Card =  Card {name: "Mountain", cardtype: "land", cmc: 0};
// pub static Plains: Card = Card {name: "Plains", cardtype: "land", cmc: 0};
// pub static Swamp: Card = Card {name: "Swamp", cardtype: "land", cmc: 0};
// pub static Island: Card = Card {name: "Island", cardtype: "land", cmc: 0};
// pub static Forest: Card = Card {name: "Forest", cardtype: "land", cmc: 0};
