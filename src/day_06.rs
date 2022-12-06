#![allow(unused)]

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

pub fn main() -> Result<()> {
    let file_name = "inputs/day_06.txt";

    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut letter_counts: HashMap<char, usize> = HashMap::new();

    // probably only one line in this question
    for line in lines {
        let letters = line.unwrap().to_string();

        // run length of 4 for first part
        let unique_run_length = 14;

        for (i, letter) in letters.chars().enumerate() {
            // add new letter if it exists
            if letter_counts.contains_key(&letter) {
                letter_counts.insert(letter, letter_counts.get(&letter).unwrap() + 1);
            } else {
                letter_counts.insert(letter, 1);
            }

            // remove back letter if exists
            if i as isize - unique_run_length >= 0 {
                let back_letter = letters
                    .chars()
                    .nth((i as isize - unique_run_length) as usize)
                    .unwrap();

                letter_counts.insert(back_letter, letter_counts.get(&back_letter).unwrap() - 1);

                // remove letter if count is 0
                if letter_counts.get(&back_letter).unwrap() == &0 {
                    letter_counts.remove(&back_letter);
                }
            }

            // break if we find a unique run
            if letter_counts.len() >= unique_run_length as usize {
                println!("Packet found at i = {}", i + 1);
                break;
            }
        }
    }
    Ok(())
}
