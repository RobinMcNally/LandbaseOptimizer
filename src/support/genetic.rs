use support::types::*;
use support::utils::{sortbyfitness, randomland, roulette_select, tournament_select};
use support::fitness::cardcolorfitness;
use rand;
use rand::distributions::{IndependentSample, Range};


fn randomlandset(pop: &mut Population, decksize: i32, population_size: i32) {
    for _ in 0..population_size {
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

fn set_population_fitness(pop: &mut Population, fixed: &Deck) {
    //Initialize Fitness
    let mut fitness_sum: f32 = 0.0;
    for x in 0..pop.decks.len() {
        pop.decks[x].fitness = callfitness(fixed, &pop.decks[x]);
        fitness_sum += pop.decks[x].fitness;
        //println!("index: {}, fitness: {}, fitness_sum: {}", x, pop.decks[x].fitness, fitness_sum);
    }

    //Normalize the fitness
    for x in 0..pop.decks.len() {
        pop.decks[x].fitness = pop.decks[x].fitness / fitness_sum;
    }
}

fn select_parents(pop: &mut Population, selection_type: &str, land_len: usize) -> (Deck, Deck) {
    match selection_type {
        "roulette" => return roulette_select(&pop),
        "tournament" => return tournament_select(&pop, land_len),
        _ => return tournament_select(&pop, land_len),
    };
}

pub fn run( fixed: &Deck,           decksize: i32,
            population_size: i32,   selection_type: &str,
            cross_chance: f32,      mut_chance: f32,
            gen_count: i32,         elitist: bool) -> Deck {
    //generate base population, random combindations of decks
    let mut rng = rand::thread_rng();
    let land_len: usize = decksize as usize - fixed.cards.len();
    let land_range = Range::new(0, land_len);
    let decimal_range = Range::new(0., 1.);

    let mut pop = Population {decks: vec![]};
    randomlandset(&mut pop, land_len as i32, population_size);

    set_population_fitness(&mut pop, fixed);

    sortbyfitness(&mut pop);
    
    //repeat for size of population
    for _ in 0..gen_count {
    // while callfitness(fixed, &pop.decks[0].clone()) < 600.0 {
        //Generate a new population from the current one
        let mut decks: Vec<Deck> = vec![];
        let mut pop_index = 0;
        if elitist {
            decks.push(pop.decks[0].clone());
            pop_index += 1;
        }
        while pop_index < population_size {
            //select two parents from the population
            let (father, mother) = select_parents(&mut pop, selection_type, land_len);

            let mut child1 = Deck{cards: vec![], fitness: 0.0};
            let mut child2 = Deck{cards: vec![], fitness: 0.0};

            //Cross them together at a random point
            if decimal_range.ind_sample(&mut rng) < cross_chance {

                // Choose the split point
                let num: usize = land_range.ind_sample(&mut rng);

                let (fleft, fright) = father.cards.split_at(num);
                let (mleft, mright) = mother.cards.split_at(num);

                child1.cards.extend(fleft.iter().cloned());
                child1.cards.extend(mright.iter().cloned());

                child2.cards.extend(mleft.iter().cloned());
                child2.cards.extend(fright.iter().cloned());
            } else {
                //println!("Not Crossing");
                child1.cards.extend(father.cards.clone());
                child2.cards.extend(mother.cards.clone());
            }
            //Mutate Child1 Maybe
            if decimal_range.ind_sample(&mut rng) < mut_chance {
                let num = land_range.ind_sample(&mut rng);
                child1.cards.push(randomland());
                child1.cards.swap_remove(num);
            }
            if decimal_range.ind_sample(&mut rng) < mut_chance {
                let num = land_range.ind_sample(&mut rng);
                child2.cards.push(randomland());
                child2.cards.swap_remove(num);
            }
            decks.push(child1.clone());
            if pop_index != population_size - 1 {
                decks.push(child2.clone());
            }
            pop_index += 2;
        }
        //Initialize Fitness
        let mut pop = Population{decks: decks};

        set_population_fitness(&mut pop, fixed);
        println!("{:?}", callfitness(fixed, &pop.decks[0].clone()));
        //Sort by fitness
        sortbyfitness(&mut pop);
    }
    return pop.decks[0].clone();
    //return pop.decks.first().unwrap();
        //Random chance to mutate
        //Put child into new population
    //if end condition is met stop and return best solution in current population
}
