#![allow(unused)]

use anyhow::{Context, Result};
use std::fs;
use std::io::{self, BufRead};

fn create_pairs(line: String) -> Result<(Vec<Vec<isize>>)> {
    let ranges = line.split(",");

    let mut pairs: Vec<Vec<isize>> = Vec::new();
    for range in ranges {
        let position: Vec<isize> = range.split("-").map(|pos| pos.parse().unwrap()).collect();

        pairs.push(position);
    }

    Ok(pairs)
}

fn compare_ranges(pairs: Vec<Vec<isize>>) -> Result<(bool)> {
    let first = pairs[0].clone();
    let last = pairs[1].clone();

    // full overlap condition
    let full_contain = (first[0] - last[0]) * (first[1] - last[1]) <= 0;

    // partial overlap condition
    let partial_contain = (first[1] - last[0]) * (last[1] - first[0]) >= 0;

    Ok(partial_contain)
}

pub fn main() -> Result<()> {
    let file_name = "inputs/day_04.txt";

    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut overlaps = 0;

    for line in lines {
        let line_str = line.unwrap();
        let pairs = create_pairs(line_str).unwrap();
        let contain = compare_ranges(pairs).unwrap();

        if contain {
            overlaps += 1;
        }
    }

    println!("The number of overlaps is: {}", overlaps);

    Ok(())
}
