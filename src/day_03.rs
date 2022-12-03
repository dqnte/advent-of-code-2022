#![allow(unused)]

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::{fs, vec};

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn create_alphabet_map() -> Result<(HashMap<char, usize>)> {
    let mut mapped_alphabet: HashMap<char, usize> = HashMap::new();

    for (i, letter) in ALPHABET.chars().enumerate() {
        mapped_alphabet.insert(letter, i + 1);
    }

    for (i, letter) in ALPHABET.to_uppercase().chars().enumerate() {
        mapped_alphabet.insert(letter, i + 27);
    }

    Ok(mapped_alphabet)
}

fn find_common_item(
    lines: Vec<String>,
    mapped_alphabet: &HashMap<char, usize>,
) -> Result<(&usize)> {
    // find all unique chars
    let mut found: HashMap<char, usize> = HashMap::new();

    for line in &lines {
        // dedup letters in line, could probably do this without sorting, *sigh*
        let mut unique_chars: Vec<char> = line.chars().collect();
        unique_chars.sort();
        unique_chars.dedup();

        for letter in unique_chars {
            // count values in string
            if found.contains_key(&letter) {
                found.insert(letter, found.get(&letter).unwrap() + 1);
            } else {
                found.insert(letter, 1);
            }

            // if count is equal to len of lines, then letter is in all strings
            if found.get(&letter).unwrap() == &lines.len() {
                return Ok(mapped_alphabet.get(&letter).unwrap());
            }
        }
    }

    Ok(&0)
}

pub fn main() -> Result<()> {
    let file_name = "inputs/day_03.txt";

    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mapped_alphabet = create_alphabet_map().unwrap();

    let mut total = 0;
    let mut line_chunks: Vec<String> = Vec::new();

    for (i, line) in lines.enumerate() {
        let line_str = line.unwrap();

        // part 1 splits
        // let line_len = line_str.len() / 2;
        // let left_side = line_str[..line_len].to_string();
        // let right_side = line_str[line_len..].to_string();
        // let both_sides: Vec<String> = vec![left_side, right_side];
        //
        // total += find_common_item(both_sides, &mapped_alphabet).unwrap();

        line_chunks.push(line_str);

        if (i + 1) % 3 == 0 {
            total += find_common_item(line_chunks.clone(), &mapped_alphabet).unwrap();
            line_chunks = Vec::new()
        }
    }

    println!("Total score: {}", total);

    Ok(())
}
