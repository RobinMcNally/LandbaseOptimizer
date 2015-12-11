use support::types::*;
use rand;
use rand::distributions::{IndependentSample, Range};

pub fn randomland() -> Card {
    let mut rng = rand::thread_rng();
    let basic_range = Range::new(1, 6);
    let nametemp;
    let typetemp;
    match basic_range.ind_sample(&mut rng){
        1 => {
            nametemp = String::from("Forest");
            typetemp = String::from("Basic Land — Forest");
        },
        2 => {
            nametemp = String::from("Mountain");
            typetemp = String::from("Basic Land — Mountain");
        },
        3 => {
            nametemp = String::from("Swamp");
            typetemp = String::from("Basic Land — Swamp");
        },
        4 => {
            nametemp = String::from("Plains");
            typetemp = String::from("Basic Land — Plains");
        },
        5 => {
            nametemp = String::from("Island");
            typetemp = String::from("Basic Land — Island");
        },
        _ => {
            nametemp = String::from("Forest");
            typetemp = String::from("Basic Land — Forest");
        },
    };
    let card = Card {name: nametemp, cardtype: typetemp, mana_cost: String::new(), colors: vec![String::new()]};
    return card;
}

//Since this will probably top out at 100 elements
//Only bubble sort temporarily
pub fn sortbyfitness(pop: &mut Population) {
    for x in 0..pop.decks.len() {
        for y in (x + 1)..pop.decks.len() {
            if pop.decks[x].fitness < pop.decks[y].fitness {
                pop.decks.swap(x, y);
            }
        }
    }
}
