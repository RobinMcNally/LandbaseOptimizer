use support::types::*;
use support::fitness::cardcolorfitness;
use rand;
use rand::distributions::{IndependentSample, Range};

const POPULATIONSIZE: i32 = 50;
const CROSSCHANCE: f32 = 0.6;
const MUTCHANCE: f32 = 0.05;
const GENERATIONS: usize = 1000;
const ELITIST: bool = true;

fn randomland() -> Card {
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

fn randomlandset(pop: &mut Population, decksize: i32) {
    for _ in 0..(POPULATIONSIZE - 1) {
        let mut deck = Deck {cards: vec![], fitness: 0.0};
        for _ in 0..decksize {
            deck.cards.push(randomland());
        }
        pop.decks.push(deck);
    }
}

//Calls various fitness functions
fn callfitness(fixed: &Deck, totest: &Deck) -> f32 {
    return cardcolorfitness(fixed, totest);
}

//Since this will probably top out at 100 elements
//Only bubble sort temporarily
fn sortbyfitness(pop: &mut Population) {
    for x in 0..pop.decks.len() {
        for y in (x + 1)..pop.decks.len() {
            if pop.decks[x].fitness < pop.decks[y].fitness {
                pop.decks.swap(x, y);
            }
        }
    }
}

pub fn run(fixed: &Deck, decksize: i32) -> Deck {
    //generate base population, random combindations of decks
    let mut rng = rand::thread_rng();
    let maxpop = decksize as usize - fixed.cards.len();
    let land_range = Range::new(0, maxpop);
    let decimal_range = Range::new(0., 1.);

    let mut pop = Population {decks: vec![]};
    //println!("maxpop: {}", maxpop);
    randomlandset(&mut pop, maxpop as i32);

    //Initialize Fitness
    let mut fitness_sum: f32 = 0.0;
    for x in 0..pop.decks.len() {
        pop.decks[x].fitness = callfitness(fixed, &pop.decks[x]);
        ////println!("{:?}", pop.decks[x]);
        ////println!("{:?}", pop.decks[x].fitness);
        fitness_sum += pop.decks[x].fitness;
        //println!("index: {}, fitness: {}, fitness_sum: {}", x, pop.decks[x].fitness, fitness_sum);
    }
    //Normalize the fitness
    for x in 0..pop.decks.len() {
        pop.decks[x].fitness = pop.decks[x].fitness / fitness_sum;
    }
    //Sort by fitness
    sortbyfitness(&mut pop);
    for x in pop.decks.clone() {
        print!(", {}", x.fitness);
    }
    println!("");
    //repeat for size of population
    for _ in 0..GENERATIONS {
        //Generate a new population from the current one
        let mut decks: Vec<Deck> = vec![];
        let mut index;
        let mut pop_index = 0;
        let mut parent_decks: Vec<usize> = vec![];
        if ELITIST {
            decks.push(pop.decks[0].clone());
            pop_index += 1;
        }
        while pop_index < POPULATIONSIZE {
            //select two parents from the population (random with weights towards higher fitness)
            for _ in 0..2 {
                let num: f32 = decimal_range.ind_sample(&mut rng);
                ////println!("{}", num);
                fitness_sum = 0.0;
                index = 0;
                //println!("num: {}", num);
                while fitness_sum < num {
                    fitness_sum += pop.decks[index].fitness;
                    //println!("index: {}, fitness_sum: {}, fitness: {}", index, fitness_sum, pop.decks[index].fitness);
                    index += 1;
                }
                parent_decks.push(index - 1);
            }
            let father = pop.decks[parent_decks[0]].clone();
            let mother = pop.decks[parent_decks[1]].clone();

            let mut child1 = Deck{cards: vec![], fitness: 0.0};
            let mut child2 = Deck{cards: vec![], fitness: 0.0};

            //Cross them together at a random point
            if decimal_range.ind_sample(&mut rng) < CROSSCHANCE {
                //println!("Crossing");
                let num: usize = land_range.ind_sample(&mut rng);

                let (fleft, fright) = father.cards.split_at(num);
                let (mleft, mright) = mother.cards.split_at(num);

                let mut vectemp = Vec::new();
                vectemp.extend(fleft.iter().cloned());
                vectemp.extend(mright.iter().cloned());
                child1.cards = vectemp;

                let mut vectemp = Vec::new();
                vectemp.extend(mleft.iter().cloned());
                vectemp.extend(fright.iter().cloned());
                child2.cards = vectemp;
            } else {
                //println!("Not Crossing");
                child1.cards.extend(father.cards.clone());
                child2.cards.extend(mother.cards.clone());
            }
            //Mutate Child1 Maybe
            if decimal_range.ind_sample(&mut rng) < MUTCHANCE {
                //println!("Mutating Child 1: Size {}", child2.cards.len());
                let num = land_range.ind_sample(&mut rng);
                child1.cards.push(randomland());
                child1.cards.swap_remove(num);
                //println!("Done Mutating");
            }
            if decimal_range.ind_sample(&mut rng) < MUTCHANCE {
                let num = land_range.ind_sample(&mut rng);
                child2.cards.push(randomland());
                //println!("Mutating Child 2: Size {}", child2.cards.len());
                child2.cards.swap_remove(num);
                // println!("Done Mutating");
            }
            decks.push(child1.clone());
            if pop_index != POPULATIONSIZE - 1 {
                decks.push(child2.clone());
            }
            pop_index += 2;
        }
        //Initialize Fitness
        let mut pop = Population {decks: decks.clone()};
        let mut fitness_sum: f32 = 0.0;
        for x in 0..pop.decks.len() {
            pop.decks[x].fitness = callfitness(fixed, &pop.decks[x]);
            fitness_sum += pop.decks[x].fitness;
        }
        //Normalize the fitness
        for x in 0..pop.decks.len() {
            pop.decks[x].fitness = pop.decks[x].fitness / fitness_sum;
        }
        //Sort by fitness
        sortbyfitness(&mut pop);
    }
    return pop.decks[0].clone();
    //return pop.decks.first().unwrap();
        //Random chance to mutate
        //Put child into new population
    //if end condition is met stop and return best solution in current population
}
