#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: usize,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.day {
        1 => day_01::main()?,
        2 => day_02::main()?,
        3 => day_03::main()?,
        4 => day_04::main()?,
        5 => day_05::main()?,
        _ => println!("No file found"),
    }

    Ok(())
}
