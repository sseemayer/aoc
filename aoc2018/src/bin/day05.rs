use std::{fs::File, io::Read};

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("I/O error: {}", _0)]
    Io(#[from] std::io::Error),

    #[error("Int parsing error: {}", _0)]
    ParseInt(#[from] std::num::ParseIntError),
}

fn react(polymer: &mut Vec<char>, start: usize) -> Option<usize> {
    let mut out: Vec<char> = Vec::new();
    let mut i = start;
    while i < polymer.len() {
        let a = polymer[i - 1];
        let b = polymer[i];

        if (a.is_lowercase() && a.to_ascii_uppercase() == b)
            || (a.is_uppercase() && a.to_ascii_lowercase() == b)
        {
            polymer.remove(i);
            polymer.remove(i - 1);
            return if i > 3 { Some(i - 2) } else { Some(1) };
        } else {
            out.push(a);
            i += 1;
        }
    }

    None
}

fn react_fully(polymer: &Vec<char>) -> Vec<char> {
    let mut data = polymer.clone();

    let mut start = 1;
    loop {
        if let Some(new_start) = react(&mut data, start) {
            start = new_start;
        } else {
            return data;
        }
    }
}

fn remove_unit(polymer: &Vec<char>, unit: char) -> Vec<char> {
    polymer
        .into_iter()
        .filter(|u| u.to_ascii_lowercase() != unit)
        .map(|u| *u)
        .collect::<Vec<char>>()
}

fn main() -> Result<()> {
    let mut f = File::open("data/day05/input").context("open input")?;
    let mut polymer = String::new();
    f.read_to_string(&mut polymer).context("read input")?;

    let polymer: Vec<char> = polymer.trim().chars().collect();
    let reacted = react_fully(&polymer).into_iter().collect::<String>();

    println!("Part 1: len={}", reacted.len());

    let mut shortest = usize::MAX;
    for remove in 'a'..='z' {
        let poly_reduced = remove_unit(&polymer, remove);
        let reacted = react_fully(&poly_reduced).into_iter().collect::<String>();

        if reacted.len() < shortest {
            shortest = reacted.len();
        }
    }

    println!("Part 2: len={}", shortest);

    Ok(())
}
