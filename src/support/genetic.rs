use support::types::*;
use rand;
use rand::distributions::{IndependentSample, Range};

const POPULATIONSIZE: i32 = 50;
const CROSSCHANCE: f32 = 0.7;
const MUTCHANCE: f32 = 1.0;
const GENERATIONS: usize = 700;

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
    return;
}
fn cardcolorfitness(fixed: &Deck, totest: &Deck) -> f32 {
    let mut costs : Vec<f32> = vec![0.0; 5];
    let mut landcounts : Vec<f32> = vec![0.0; 5];

    for x in fixed.clone().cards {
        for y in 0..(x.colors.len()) {
            match &(x.colors.get(y).unwrap())[..] {
                "White" => costs[0] += 1.0,
                "Blue"  => costs[1] += 1.0,
                "Black" => costs[2] += 1.0,
                "Red"   => costs[3] += 1.0,
                "Green" => costs[4] += 1.0,
                _ => println!("Unexpected performance: costs"),
            }
        }
    }

    for x in totest.clone().cards {
        match &(x.name)[..] {
            "Plains"    => landcounts[0] += 1.0,
            "Island"    => landcounts[1] += 1.0,
            "Swamp"     => landcounts[2] += 1.0,
            "Mountain"  => landcounts[3] += 1.0,
            "Forest"    => landcounts[4] += 1.0,
            _ => println!("Unexpected performance: landcounts"),
        };
    }

    for x in 0..5 {
        if costs[x] != 0.0 {
            println!("landcounts: {}, costs: {}", landcounts[x], costs[x]);
            costs[x] = landcounts[x] / costs[x];
        }
    }
    let mut sum = 0.0;
    for x in costs {
            sum += x;
    }
    println!("Sum: {}", sum);
    return sum;
}
//Calls various fitness functions
fn callfitness(fixed: &Deck, totest: &Deck) -> f32 {
    return cardcolorfitness(fixed, totest);
}

//Since this will probably top out at 100 elements
//Only bubble sort temporarily
fn sortbyfitness(pop: &mut Population) {
    for x in 0..pop.decks.len() {
        for y in (x + 1)..(pop.decks.len()-1) {
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
    println!("maxpop: {}", maxpop);
    randomlandset(&mut pop, maxpop as i32);

    //Initialize Fitness
    let mut fitness_sum: f32 = 0.0;
    for x in 0..pop.decks.len() {
        pop.decks[x].fitness = callfitness(fixed, &pop.decks[x]);
        println!("{:?}", pop.decks[x]);
        println!("{:?}", pop.decks[x].fitness);
        fitness_sum += pop.decks[x].fitness;
    }
    //Normalize the fitness
    for x in 0..pop.decks.len() {
        pop.decks[x].fitness = pop.decks[x].fitness / fitness_sum;
    }
    //Sort by fitness
    sortbyfitness(&mut pop);

    //repeat for size of population
    for _ in 0..GENERATIONS {
        //Generate a new population from the current one
        let mut decks: Vec<Deck> = vec![];
        let mut index = 0;
        let mut pop_index = 0;
        let mut parent_decks: Vec<usize> = vec![];
        while pop_index < POPULATIONSIZE {
            //select two parents from the population (random with weights towards higher fitness)
            for _ in 0..2 {
                let num: f32 = decimal_range.ind_sample(&mut rng);
                //println!("{}", num);
                fitness_sum = 0.0;
                while fitness_sum < num {
                    fitness_sum += pop.decks[index].fitness;
                    index += 1;
                }
                parent_decks.push(index - 1);
            }
            let father = pop.decks.get(parent_decks[0]).unwrap().clone();
            let mother = pop.decks.get(parent_decks[1]).unwrap().clone();

            let mut child1 = Deck{cards: vec![], fitness: 0.0};
            let mut child2 = Deck{cards: vec![], fitness: 0.0};

            //Cross them together at a random point
            if decimal_range.ind_sample(&mut rng) < CROSSCHANCE {
                println!("Crossing");
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
                println!("Not Crossing");
                let mut child1 = Deck{cards: vec![], fitness: 0.0};
                child1.cards = father.cards.clone();
                let mut child2 = Deck{cards: vec![], fitness: 0.0};
                child2.cards = mother.cards.clone();
            }
            //Mutate Child1 Maybe
            if decimal_range.ind_sample(&mut rng) < MUTCHANCE {
                println!("Mutating Child 1: Size {}", child2.cards.len());
                let num = land_range.ind_sample(&mut rng);
                child1.cards.push(randomland());
                child1.cards.swap_remove(num);
                println!("Done Mutating");
            }
            if decimal_range.ind_sample(&mut rng) < MUTCHANCE {
                println!("Mutating Child 2: Size {}", child2.cards.len());
                let num = land_range.ind_sample(&mut rng);
                child2.cards.push(randomland());
                child2.cards.swap_remove(num);
                println!("Done Mutating");
            }
            decks.push(child1.clone());
            if pop_index != POPULATIONSIZE - 1 {
                decks.push(child2.clone());
            }
            pop_index += 2;
        }
        //Initialize Fitness
        let mut pop = Population {decks: decks};
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
    return pop.decks.first().unwrap().clone();
    //return pop.decks.first().unwrap();
        //Random chance to mutate
        //Put child into new population
    //if end condition is met stop and return best solution in current population
}
