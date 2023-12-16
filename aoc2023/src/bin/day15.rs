use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Error, Result};

fn hash(s: &str) -> usize {
    let mut cur = 0;

    for c in s.chars() {
        cur += (c as u8) as usize;
        cur *= 17;
        cur %= 256;
    }

    cur
}

#[derive(Debug, Clone)]
enum Operation {
    Remove(String),
    Add(String, usize),
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        if s.ends_with("-") {
            let label = s.trim_end_matches("-").to_string();
            Ok(Operation::Remove(label))
        } else {
            let (label, focal_length) = s
                .split_once("=")
                .ok_or_else(|| anyhow!("Bad add operation: '{}'", s))?;
            Ok(Operation::Add(label.to_string(), focal_length.parse()?))
        }
    }
}

#[derive(Debug, Default)]
struct State {
    boxes: HashMap<usize, Vec<(String, usize)>>,
}

impl State {
    fn step(&mut self, op: &Operation) {
        match op {
            Operation::Remove(label) => {
                let box_index = hash(label);
                let b = self.boxes.entry(box_index).or_default();

                for (i, (l, _f)) in b.iter().enumerate() {
                    if l == label {
                        b.remove(i);
                        break;
                    }
                }
            }
            Operation::Add(label, focal_length) => {
                let box_index = hash(label);
                let b = self.boxes.entry(box_index).or_default();

                for (l, f) in b.iter_mut() {
                    if l == label {
                        *f = *focal_length;
                        return;
                    }
                }

                b.push((label.to_string(), *focal_length));
            }
        }
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(|(j, (_label, focal_length))| (i + 1) * (j + 1) * focal_length)
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() -> Result<()> {
    let path = "data/day15/input";

    let sum: usize = aoc::io::read_all(path)?.trim().split(",").map(hash).sum();

    println!("Part 1: {}", sum);

    let operations: Vec<Operation> = aoc::io::read_all(path)?
        .split(",")
        .map(|o| o.parse())
        .collect::<Result<Vec<Operation>>>()?;

    let mut state = State::default();

    for op in &operations {
        state.step(op);
    }

    println!("Part 2: {}", state.focusing_power());

    Ok(())
}
