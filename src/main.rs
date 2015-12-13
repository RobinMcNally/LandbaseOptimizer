extern crate rustc_serialize;
extern crate rand;
use rustc_serialize::json::Json;
use std::fs::File;
use std::env;

use std::io::Read;
use support::types::Deck;
use support::types::Card;
use support::genetic;
pub mod support;

fn main() {
    let args : Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("usage: {} filename", args[0]);
        return;
    }

    let mut file = File::open("res/AllCards.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json = Json::from_str(&data).unwrap();

    let mut static_cards = File::open(&args[1]).unwrap();
    let mut raw = String::new();

    static_cards.read_to_string(&mut raw).unwrap();
    let split = raw.split("\n");

    let mut series = Deck {cards: vec![], fitness: 0.0};
    for x in split {
        let s = match json.find(&x) {
            None => break,
            Some(i) => i,
        };
        let nametemp = match s.find("name"){
            None => {println!("Yo name's broke"); return },
            Some(i) => i,
        };
        let typetemp = match s.find("type"){
            None => {println!("Yo type's broke"); return },
            Some(i) => i,
        };
        let colors = match s.find("colors"){
            None => {println!("Yo color's broke"); return },
            Some(i) => i,
        };

        let arr = colors.as_array().unwrap();
        let mut stringarr: Vec<String> = vec![];

        for x in 0..arr.len() {
            let mut strtemp = arr.get(x).unwrap().to_string();
            strtemp.remove(0);
            strtemp.pop();
            stringarr.push(strtemp);
        }

        let card = Card { name: nametemp.to_string(), cardtype : typetemp.to_string(), mana_cost : String::new(), colors: stringarr};
        series.cards.push(card);
    }

    //popiterator: 100, selectchance: 0.3, mutchance: 0.08, generation_size: 500, tournament, elite
    //13/24 were mountains

    println!("{:?}", genetic::run(&series, 60, 100, "elite", 0.3, 0.08, 500, true));

    // let mut best_vec: Vec<(Deck, f32)> = vec![];
    // let mut description_vec: Vec<String> = vec![];
    // //Lets find some convergance!
    // for selectchance in 1..10{
    //     for mutchance in 1..12 {
    //             let string_temp: String = format!("popiterator: {}, selectchance: {}, mutchance: {}, generation_size: {}, tournament, elite",
    //                 100, (selectchance as f32 * 0.05), (mutchance as f32 * 0.01), 500);
    //             description_vec.push(string_temp);
    //             let (a, b) = genetic::run(&series, 60,
    //                 100, "tournament",
    //                 (selectchance as f32 * 0.1),
    //                 (mutchance as f32 * 0.02),
    //                 500, true);
    //             best_vec.push( (a, b) );
    //     }
    // }
    // let mut best_fit: f32 = 0.0;
    // let mut best_description: String = String::new();
    // let mut best: Deck = Deck{cards: vec![], fitness: 0.0};
    // for x in 0..best_vec.len() {
    //     if best_vec[x].1 > best_fit {
    //         best_fit = best_vec[x].1;
    //         best = best_vec[x].0.clone();
    //         best_description = description_vec[x].clone();
    //     }
    // }
    //
    // println!("Best Fitness Found: \n{}\nFitnesss: {}\n{:?}", best_description, best_fit, best);
}
