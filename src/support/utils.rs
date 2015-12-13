use support::types::*;
use rand;
use rand::distributions::{IndependentSample, Range};

//Tournament size as a fraction of the whole
//ie. the real number use for the tournament is
// population_size / TOURNAMENTSIZE
const TOURNAMENTFRAC: i32 = 4;

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

//Selection Functions
//Note the pop variable is expected to be implemented
pub fn roulette_select(pop: &Population) -> (Deck, Deck) {
    let mut rng = rand::thread_rng();
    let decimal_range = Range::new(0., 1.);
    let mut parent_decks: Vec<usize> = vec![];
    for _ in 0..2 {
        let num: f32 = decimal_range.ind_sample(&mut rng);
        let mut fitness_sum: f32 = 0.0;
        let mut index: usize = 0;
        while fitness_sum <= num {
            fitness_sum += pop.decks[index].fitness;
            index += 1;
        }
        parent_decks.push(index - 1);
    }
    let father = pop.decks[parent_decks[0]].clone();
    let mother = pop.decks[parent_decks[1]].clone();

    return (father.clone(), mother.clone());
}

pub fn tournament_select(pop: &Population) -> (Deck, Deck) {
    let mut rng = rand::thread_rng();
    let pop_range = Range::new(0, pop.decks.len());
    let mut return_pop: Population = Population {decks: vec![]};
    let loop_size: usize = (pop.decks.len() as i32 / TOURNAMENTFRAC) as usize;
    for _ in 0..loop_size {
        let num = pop_range.ind_sample(&mut rng);
        return_pop.decks.push(pop.decks[num].clone());
    }
    sortbyfitness(&mut return_pop);
    return (return_pop.decks[0].clone(), return_pop.decks[0].clone());
}
