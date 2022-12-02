#![allow(unused)]

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

// A,X is rock
// B,Y is paper
// C,Z is scissors

pub fn main() -> Result<()> {
    let file_name = "inputs/day_02.txt";

    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut total = 0;

    // let pair_map: HashMap<&str, usize> = HashMap::from([
    //     ("A X", 3),
    //     ("A Y", 6),
    //     ("A Z", 0),
    //     ("B X", 0),
    //     ("B Y", 3),
    //     ("B Z", 6),
    //     ("C X", 6),
    //     ("C Y", 0),
    //     ("C Z", 3),
    // ]);
    let pair_map: HashMap<&str, usize> = HashMap::from([
        ("A X", 3),
        ("A Y", 1),
        ("A Z", 2),
        ("B X", 1),
        ("B Y", 2),
        ("B Z", 3),
        ("C X", 2),
        ("C Y", 3),
        ("C Z", 1),
    ]);


    // let choice_map: HashMap<&str, usize> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let choice_map: HashMap<&str, usize> = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    for line in lines {
        let line_str = line.unwrap();
        let my_choice = line_str.split(" ").last().unwrap();

        // use [..] to convert String into &str
        total += pair_map.get(&line_str[..]).unwrap();
        total += choice_map.get(&my_choice[..]).unwrap();
    }

    println!("Total score: {}", total);

    Ok(())
}
