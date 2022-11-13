use std::{
    collections::HashMap,
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

fn count_letters(s: &str) -> HashMap<char, usize> {
    let mut out = HashMap::new();
    for c in s.chars() {
        *out.entry(c).or_default() += 1;
    }
    out
}

fn swap_counts(counts: HashMap<char, usize>) -> HashMap<usize, Vec<char>> {
    let mut out: HashMap<usize, Vec<char>> = HashMap::new();

    for (k, v) in counts.into_iter() {
        out.entry(v).or_default().push(k);
    }

    out
}

fn edit_distance(a: &str, b: &str) -> usize {
    let mut out = 0;
    for (a, b) in a.chars().zip(b.chars()) {
        if a != b {
            out += 1
        }
    }

    out
}

fn common_substring(a: &str, b: &str) -> String {
    let mut out = String::new();
    for (a, b) in a.chars().zip(b.chars()) {
        if a == b {
            out.push(a);
        }
    }
    out
}

fn main() -> Result<()> {
    let ids = BufReader::new(File::open("data/day02/input").context("Opening input")?)
        .lines()
        .map(|l| l.context("Read line"))
        .collect::<Result<Vec<String>>>()?;

    let counts = ids
        .iter()
        .map(|l| Ok(swap_counts(count_letters(l))))
        .collect::<Result<Vec<HashMap<usize, Vec<char>>>>>()?;

    let two_of_a_letter = counts
        .iter()
        .filter(|c| c.get(&2).map(|v| v.len()).unwrap_or_default() >= 1)
        .count();

    let three_of_a_letter = counts
        .iter()
        .filter(|c| c.get(&3).map(|v| v.len()).unwrap_or_default() >= 1)
        .count();

    println!("Part 1: {}", two_of_a_letter * three_of_a_letter);

    for (i, a) in ids.iter().enumerate() {
        for b in ids.iter().take(i) {
            if edit_distance(a, b) == 1 {
                let css = common_substring(a, b);
                println!("Part 2: {}", css);
            }
        }
    }

    Ok(())
}
