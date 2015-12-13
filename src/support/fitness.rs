use support::types::*;
use rand;
use rand::distributions::{IndependentSample, Range};


const DISDAIN_CONSTANT: f32 = -30.0;
const FAVOR_CONSTANT: f32 = 30.0;

pub fn cardcolorfitness(fixed: &Deck, totest: &Deck) -> f32 {
    let mut costs : Vec<f32> = vec![0.0; 5];
    let mut landcounts : Vec<f32> = vec![0.0; 5];
    let mut sum = 0.0;

    for x in fixed.clone().cards {
        for y in 0..(x.colors.len()) {
            match &(x.colors.get(y).unwrap())[..] {
                "White" => costs[0] += 1.0,
                "Blue"  => costs[1] += 1.0,
                "Black" => costs[2] += 1.0,
                "Red"   => costs[3] += 1.0,
                "Green" => costs[4] += 1.0,
                _ => println!("Unexpected performance: costs"),
            };
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
            ////println!("landcounts: {}, costs: {}", landcounts[x], costs[x]);
            costs[x] = (landcounts[x] / costs[x]) * FAVOR_CONSTANT;
        }
    }
    for x in costs {
            sum += x;
    }
    return sum;
}

pub fn cardcolorfitnesssum(fixed: &Deck, totest: &Deck) -> f32 {
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
            };
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
            ////println!("landcounts: {}, costs: {}", landcounts[x], costs[x]);
            costs[x] = (landcounts[x] / costs[x]) * FAVOR_CONSTANT;
        } else if landcounts[x] > 0.0 {
            costs[x] = DISDAIN_CONSTANT * landcounts[x];
        }
    }
    let mut sum = 1000.0;
    //println!("{:?}", costs);
    for x in costs {
            sum += x;
    }
    ////println!("Sum: {}", sum);
    return sum;
}
