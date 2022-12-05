#![allow(unused)]

use anyhow::{Context, Result};
use std::fs;
use std::io::{self, BufRead};

fn build_stacks() -> Result<(Vec<Vec<char>>)> {
    let file_name = "inputs/day_05_stack.txt";

    let file = fs::File::open(file_name).unwrap();
    let mut lines: Vec<_> = io::BufReader::new(file).lines().collect();
    lines.reverse();

    let mut num_stacks = 0;
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let line_str = line.as_ref().unwrap().to_string();

        if i == 0 {
            // number of stacks is last value in column
            num_stacks = line_str.split(" ").last().unwrap().parse().unwrap();

            // initialize empty lists
            let mut empty_stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];
            stacks.append(&mut empty_stacks);
        } else {
            for idx in 0..num_stacks {
                // check if column has crate
                let col_idx = (idx * 4) + 1;

                if col_idx >= line_str.len() {
                    break;
                }

                let crate_char = line_str.chars().nth(col_idx).unwrap();

                if !crate_char.is_whitespace() {
                    stacks[idx].push(crate_char);
                }
            }
        }
    }

    Ok(stacks)
}

fn move_creates(
    start_stack: usize,
    end_stack: usize,
    num_moves: usize,
    stacks: &mut Vec<Vec<char>>,
) -> Result<()> {

    // move crates as a stack
    // let new_len = stacks[start_stack - 1].len() - num_moves;
    // let mut moving = stacks[start_stack - 1].split_off(new_len);
    // moving.reverse();
    // stacks[end_stack - 1].append(&mut moving);


    // move in slices
    let new_len = stacks[start_stack - 1].len() - num_moves;
    let mut moving = stacks[start_stack - 1].split_off(new_len);
    stacks[end_stack - 1].append(&mut moving);

    Ok(())
}

fn read_instructions(stacks: &mut Vec<Vec<char>>) -> Result<()> {
    let file_name = "inputs/day_05_moves.txt";

    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        // sample line: move 1 from 2 to 1
        let line_str = line.unwrap();
        let words: Vec<&str> = line_str.split(" ").collect();

        let num_moves: usize = words[1].parse().unwrap();
        let start_stack: usize = words[3].parse().unwrap();
        let end_stack: usize = words[5].parse().unwrap();

        move_creates(start_stack, end_stack, num_moves, stacks);
    }

    Ok(())
}

pub fn main() -> Result<()> {
    // build stacks
    let mut stacks = build_stacks().unwrap();

    // process inputes
    read_instructions(&mut stacks);

    // find top values
    let mut tops: Vec<char> = Vec::new();
    for mut stack in stacks {
        tops.push(stack.pop().unwrap());
    }

    println!("The top crates are: {}", tops.iter().collect::<String>());

    Ok(())
}
