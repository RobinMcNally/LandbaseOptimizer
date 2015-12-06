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
    println!("Running Genetic");
    let result : Deck = genetic::run(&series, 60);
    println!("The Deck: {:?}", result.cards);
    println!("Genetic Done");
}
