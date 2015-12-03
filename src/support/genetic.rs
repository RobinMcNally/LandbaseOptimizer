use support::types::*;
use rand;
use rand::distributions::{IndependentSample, Range};

const POPULATIONSIZE: i32 = 12;
const CROSSCHANCE: f32 = 0.7;
const MUTCHANCE: f32 = 0.1;

fn randomcardset(pop: &mut Population, decksize: i32) {
    let mut rng = rand::thread_rng();
    let mut nametemp;
    let mut typetemp;
    let basic_range = Range::new(1, 6);
    for _ in 0..(POPULATIONSIZE - 1) {
        let mut deck = Deck {cards: vec![], fitness: 0.0};
        for _ in 0..decksize {
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
                _ => return,
            };
            let card = Card {name: nametemp, cardtype: typetemp, mana_cost: String::new()};
            deck.cards.push(card);
        }
        pop.decks.push(deck);
    }
    return;
}

fn callfitness(totest: &Deck) -> f32 {
    return 1.0;
}

//Since this will probably top out at 100 elements
//Only bubble sort temporarily
fn sortbyfitness(pop: &mut Population) {
    for x in 0..(pop.decks.len()-1) {
        for y in (x + 1)..(pop.decks.len()-1) {
            if pop.decks[x].fitness < pop.decks[y].fitness {
                pop.decks.swap(x, y);
            }
        }
    }
}

pub fn run(fixed: &Deck, decksize: i32) {
    //generate base population, random combindations of decks
    let mut rng = rand::thread_rng();
    let decimal_range = Range::new(0., 1.);
    let maxpop = decksize as usize - fixed.cards.len();
    let land_range = Range::new(0, maxpop);

    let mut pop = Population {decks: vec![]};
    let mut new_pop = Population {decks: vec![]};
    randomcardset(&mut pop, decksize);


    //Initialize Fitness
    let mut fitness_sum: f32 = 0.0;
    for x in 0..(pop.decks.len() - 1) {
        pop.decks[x].fitness = callfitness(&pop.decks[x]);
        fitness_sum += pop.decks[x].fitness;
    }
    //Normalize the fitness
    for x in 0..(pop.decks.len() - 1) {
        pop.decks[x].fitness = pop.decks[x].fitness / fitness_sum;
    }
    //Sort by fitness
    sortbyfitness(&mut pop);

    let mut index = 0;
    let mut pop_index = 0;
    let mut parent_decks: Vec<usize> = vec![];
    fitness_sum = 0.0;
    //repeat for size of population
    loop {
        //Generate a new population from the current one
        let mut decks: Vec<Deck> = vec![];
        while pop_index < POPULATIONSIZE {
            //select two parents from the population (random with weights towards higher fitness)
            for _ in 0..2 {
                let num: f32 = decimal_range.ind_sample(&mut rng);
                println!("{}", num);
                while fitness_sum < num {
                    fitness_sum += pop.decks[index].fitness;
                    index += 1;
                }
                println!("Loop done");
                parent_decks.push(index - 1);
            }
            let father = pop.decks.get(parent_decks[0]).unwrap().clone();
            let mother = pop.decks.get(parent_decks[1]).unwrap().clone();

            let mut child1 = Deck{cards: vec![], fitness: 0.0};
            let mut child2 = Deck{cards: vec![], fitness: 0.0};

            //Cross them together at a random point
            if decimal_range.ind_sample(&mut rng) < CROSSCHANCE {
                println!("Trying to cross");
                let num: usize = land_range.ind_sample(&mut rng);
                println!("Num: {}", num);
                let (fleft, fright) = father.cards.split_at(num);
                let (mleft, mright) = mother.cards.split_at(num);
                println!("{:?}", fleft);
                let mut vectemp = Vec::new();
                //father.cards = fleft;
                vectemp.extend(fleft.iter().cloned());
                vectemp.extend(mright.iter().cloned());
                //println!("Vector {:?}", vectemp);

                child1.cards = vectemp;
                let mut vectemp = Vec::new();
                vectemp.extend(mleft.iter().cloned());
                vectemp.extend(fright.iter().cloned());

                child2.cards = vectemp;
            } else {
                let mut child1 = Deck{cards: vec![], fitness: 0.0};
                child1.cards = father.cards.clone();
                let mut child2 = Deck{cards: vec![], fitness: 0.0};
                child2.cards = mother.cards.clone();
            }
            if decimal_range.ind_sample(&mut rng) < MUTCHANCE {

            }
            decks.push(child1.clone());
            if pop_index != POPULATIONSIZE - 1 {
                decks.push(child2.clone());
            }
            pop_index += 2;
        }
    }
        //Random chance to mutate
        //Put child into new population
    //if end condition is met stop and return best solution in current population
}
