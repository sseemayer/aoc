use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

#[derive(Debug)]
struct Sequence {
    numbers: Vec<isize>,
}

impl FromStr for Sequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let numbers = s
            .split_whitespace()
            .map(|n| n.parse().context("Parse sequence digits"))
            .collect::<Result<Vec<isize>>>()?;

        Ok(Self { numbers })
    }
}

impl Sequence {
    fn analyze(&self) -> Result<AnalyzedSequence> {
        let mut layers = Vec::new();
        let mut current = self.numbers.clone();

        while !current.is_empty() {
            let deltas: Vec<isize> = (1..current.len())
                .map(|i| current[i] - current[i - 1])
                .collect();

            layers.push(current[0]);

            if deltas.iter().all(|n| *n == 0) {
                return Ok(AnalyzedSequence { layers });
            }

            current = deltas;
        }

        bail!(
            "Cannot analyze sequence: {:?} after {} layers",
            self.numbers,
            layers.len()
        );
    }
}

#[derive(Debug)]
struct AnalyzedSequence {
    layers: Vec<isize>,
}

impl AnalyzedSequence {
    fn number_at(&self, pos: usize) -> isize {
        let mut current = self.layers.clone();

        for _ in 0..pos {
            current = current
                .iter()
                .enumerate()
                .map(|(j, c)| {
                    if j + 1 < current.len() {
                        c + current[j + 1]
                    } else {
                        *c
                    }
                })
                .collect();
        }

        current[0]
    }

    fn interpolate_backwards(&self) -> isize {
        let mut last = 0;
        let mut layers = self.layers.clone();

        for n in layers.iter_mut().rev() {
            *n -= last;
            last = *n;
        }

        layers[0]
    }
}

fn main() -> Result<()> {
    let mut sequences: Vec<Sequence> = aoc::io::read_lines("data/day09/input")?;

    let mut sum_forward = 0;
    let mut sum_backward = 0;
    for seq in sequences.iter_mut() {
        let analyzed = seq.analyze()?;
        let next = analyzed.number_at(seq.numbers.len());
        let prev = analyzed.interpolate_backwards();
        sum_forward += next;
        sum_backward += prev;
        // println!(
        //     "{:?} -> {:?}  next: {} prev: {}",
        //     seq.numbers, analyzed, next, prev
        // );
    }

    println!("Part 1: {}", sum_forward);
    println!("Part 2: {}", sum_backward);

    Ok(())
}
