use support::types::*;
use rand;
use rand::distributions::{IndependentSample, Range};

const POPULATIONSIZE: i32 = 12;
const CROSSCHANCE: f32 = 0.7;
const MUTCHANCE: f32 = 0.1;


fn randomcardset(pop: &mut Population) {
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

pub fn run(fixed: Deck, decksize: i32) {
    //generate base population, random combindations of decks
    let mut rng = rand::thread_rng();
    let decimal_range = Range::new(0., 1.);
    let maxpop = decksize as usize - fixed.cards.len();
    let land_range = Range::new(0, maxpop);

    let mut pop = Population {decks: vec![]};
    let mut newPop = Population {decks: vec![]};
    randomcardset(&mut pop);


    //Initialize Fitness
    let mut fitnessSum: f32 = 0.0;
    for x in 0..(pop.decks.len() - 1) {
        pop.decks[x].fitness = callfitness(&pop.decks[x]);
        fitnessSum += pop.decks[x].fitness;
    }
    //Normalize the fitness
    for x in 0..(pop.decks.len() - 1) {
        pop.decks[x].fitness = pop.decks[x].fitness / fitnessSum;
    }
    //Sort by fitness
    sortbyfitness(&mut pop);

    let mut index = 0;
    let mut popIndex = 0;
    let mut parent_decks: Vec<usize> = vec![];
    let mut father;
    let mut mother;
    fitnessSum = 0.0;
    //repeat for size of population
    loop {
        //Generate a new population from the current one
        while popIndex < POPULATIONSIZE {
            //select two parents from the population (random with weights towards higher fitness)
            for x in 0..1 {
                let mut num: f32 = decimal_range.ind_sample(&mut rng);
                println!("{}", num);
                while fitnessSum < num {
                    fitnessSum += pop.decks[index].fitness;
                    index += 1;
                }
                parent_decks.push(index - 1);
            }
                father = &pop.decks[parent_decks[0]];
                mother = &pop.decks[parent_decks[1]];
            //Cross them together at a random point
            if (decimal_range.ind_sample(&mut rng) < CROSSCHANCE) {
                let mut num: usize = land_range.ind_sample(&mut rng);
            }
            if (decimal_range.ind_sample(&mut rng) < MUTCHANCE) {

            }
            //newPop.decks[popIndex] = ;//First Child
            if popIndex != POPULATIONSIZE - 1 {
                //newPop.decks[popIndex + 1] = ;//Second
            }
        }
    }
        //Random chance to mutate
        //Put child into new population
    //if end condition is met stop and return best solution in current population
}
