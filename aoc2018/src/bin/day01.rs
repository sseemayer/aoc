use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("I/O error: {}", _0)]
    Io(#[from] std::io::Error),

    #[error("Int parsing error: {}", _0)]
    ParseInt(#[from] std::num::ParseIntError),
}

fn main() -> Result<()> {
    let numbers = BufReader::new(File::open("data/day01/input").context("Opening file")?)
        .lines()
        .map(|l| {
            l.context("Reading line")?
                .parse::<i64>()
                .context("Parsing number")
        })
        .collect::<Result<Vec<i64>>>()
        .context("reading numbers")?;

    let mut seen = HashSet::new();
    let mut sum = 0;

    for &n in numbers.iter() {
        sum += n;
        seen.insert(sum);
    }

    println!("Part 1: {}", sum);

    loop {
        for &n in numbers.iter() {
            sum += n;
            if seen.contains(&sum) {
                println!("Part 2: {}", sum);
                return Ok(());
            }
            seen.insert(sum);
        }
    }
}
