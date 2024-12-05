use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug, Clone)]
struct Input {
    dependencies: HashMap<usize, HashSet<usize>>,
    sequences: Vec<Vec<usize>>,
}

impl Input {
    fn is_valid_sequence(&self, sequence: &[usize]) -> bool {
        let mut seen: HashSet<usize> = HashSet::new();
        let subset: HashSet<usize> = sequence.iter().cloned().collect();

        for &a in sequence {
            if let Some(depends) = self.dependencies.get(&a) {
                if !depends.intersection(&subset).all(|n| seen.contains(n)) {
                    return false;
                }
            }

            seen.insert(a);
        }

        true
    }

    fn make_valid_sequence(&self, sequence: &[usize]) -> Option<Vec<usize>> {
        let subset: HashSet<usize> = sequence.iter().cloned().collect();
        let mut deps: HashMap<usize, HashSet<usize>> = self
            .dependencies
            .iter()
            .filter_map(|(k, v)| {
                if subset.contains(k) {
                    Some((
                        *k,
                        v.iter().filter(|&n| subset.contains(n)).cloned().collect(),
                    ))
                } else {
                    None
                }
            })
            .collect();

        let mut sequence: Vec<usize> = Vec::new();
        while let Some(next) = deps
            .iter()
            .find_map(|(b, a)| if a.is_empty() { Some(*b) } else { None })
        {
            sequence.push(next);
            deps.remove(&next);

            for (_, incoming) in deps.iter_mut() {
                incoming.remove(&next);
            }
        }

        if deps.is_empty() {
            Some(sequence)
        } else {
            None
        }
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut dependencies: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut sequences: Vec<Vec<usize>> = Vec::new();

        for line in s.lines() {
            if line.contains('|') {
                let (a, b) = line.split_once('|').ok_or(anyhow!("expected delimiter"))?;

                let a: usize = a.parse().context("parse page number")?;
                let b: usize = b.parse().context("parse page number")?;

                dependencies.entry(a).or_default();
                dependencies.entry(b).or_default().insert(a);
            } else if line.contains(',') {
                let sequence = line
                    .split(',')
                    .map(|n| n.parse().context("Parse sequence number"))
                    .collect::<Result<Vec<_>>>()?;
                sequences.push(sequence);
            }
        }

        Ok(Self {
            dependencies,
            sequences,
        })
    }
}

fn part1(input: &Input) -> Result<()> {
    let mut sum = 0;
    for seq in &input.sequences {
        if input.is_valid_sequence(&seq[..]) {
            sum += seq[(seq.len() - 1) / 2];
        }
    }

    println!("Part 1: {}", sum);

    Ok(())
}

fn part2(input: &Input) -> Result<()> {
    let mut sum = 0;

    for seq in &input.sequences {
        if !input.is_valid_sequence(seq) {
            let valid = input
                .make_valid_sequence(seq)
                .ok_or(anyhow!("Could not solve graph - is it cyclic?"))?;
            sum += valid[(valid.len() - 1) / 2];
        }
    }

    println!("Part 2: {}", sum);

    Ok(())
}

fn main() -> Result<()> {
    let input: Input = aoc::io::read_all((2024, 05))?.parse()?;
    //let input: Input = aoc::io::read_all("data/day05/example")?.parse()?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}
