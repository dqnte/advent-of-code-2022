#![allow(unused)]

use anyhow::{Context, Result};
use std::fs;
use std::io::{self, BufRead};

/// A function that reads a file with elf calories on every line split by blank lines,
/// and finds the sum of the calories of the top N elfs
pub fn main() -> Result<()> {
    let file_name = "inputs/day_01.txt";
    let num_values = 3;

    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut totals: Vec<usize> = Vec::new();
    let mut cur_max: usize = 0;

    for line in lines {
        let line_str = line.unwrap();

        if line_str.is_empty() {
            totals.push(cur_max);
            cur_max = 0
        } else {
            cur_max += line_str.parse::<usize>().unwrap();
        }
    }

    // one last push in case the files ends in a number
    totals.push(cur_max);

    totals.sort();
    totals.reverse();
    totals.truncate(num_values);
    let sum: usize = totals.iter().sum();

    println!("{}", sum);

    Ok(())
}
