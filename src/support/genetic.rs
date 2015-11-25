use support::types::*;
use rand;
use rand::distributions::{IndependentSample, Range};

fn randomcardset(setsize: i32, pop: &mut Population) {
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

pub fn run(fixed: Deck, decksize: i32, deckcount: i32) {
    //generate base population, random combindations of decks
    let mut rng = rand::thread_rng();
    let range = Range::new(0, 1);

    let mut pop = Population {decks: vec![]};
    randomcardset(deckcount, &mut pop);


    //Initialize Fitness
    let mut fitnessSum: f32 = 0.0;
    for mut x in 0..(pop.decks.len() - 1) {
        pop.decks[x].fitness = callfitness(&pop.decks[x]);
        fitnessSum += pop.decks[x].fitness;
    }
    //Normalize the fitness
    for mut x in 0..(pop.decks.len() - 1) {
        pop.decks[x].fitness = pop.decks[x].fitness / fitnessSum;
    }
    //Sort by fitness
    sortbyfitness(&mut pop);

    //repeat for size of population
    while true {
        //select two parents from the population (random with weights towards higher fitness)
        let mut parents = 0;
        let mut parent_decks: Vec<&mut Deck> = vec![];
        while parents != 2 {

        }
    }
        //Cross them together at a random point
        //Random chance to mutate
        //Put child into new population
    //if end condition is met stop and return best solution in current population
}
