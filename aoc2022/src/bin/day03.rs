use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Result};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Item(char);

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Item {
    fn priority(&self) -> usize {
        match self.0 {
            'a'..='z' => (self.0 as u8 - 'a' as u8) as usize + 1,
            'A'..='Z' => (self.0 as u8 - 'A' as u8) as usize + 27,
            _ => panic!("Bad item: {}", self.0),
        }
    }
}

struct Rucksack {
    compartments: Vec<Vec<Item>>,
}

impl std::fmt::Debug for Rucksack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.compartments
                .iter()
                .map(|comp| comp.iter().map(|i| i.0).collect::<String>())
                .collect::<Vec<String>>()
                .join(" / ")
        )
    }
}

impl Rucksack {
    fn from_items(items: &[Item], n_compartments: usize) -> Result<Self> {
        if items.len() % n_compartments != 0 {
            bail!("Cannot split into {} items: {:?}", n_compartments, items);
        }

        let chunk_size = items.len() / n_compartments;
        let mut compartments = Vec::new();

        for i in 0..n_compartments {
            compartments.push(items[i * chunk_size..(i + 1) * chunk_size].to_vec());
        }

        Ok(Self { compartments })
    }

    fn find_shared(&self) -> HashSet<Item> {
        self.compartments
            .iter()
            .map(|comp| comp.iter().map(|i| *i).collect::<HashSet<Item>>())
            .reduce(|accum, item| &accum & &item)
            .unwrap_or_default()
    }
}

fn parse(path: &str, n_compartments: usize) -> Result<Vec<Rucksack>> {
    let mut out = Vec::new();

    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;
        let line = line.trim().chars().map(|c| Item(c)).collect::<Vec<_>>();
        out.push(Rucksack::from_items(&line[..], n_compartments)?);
    }

    Ok(out)
}

fn part1(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .iter()
        .map(|r| r.find_shared().into_iter())
        .flatten()
        .map(|i| i.priority())
        .sum::<usize>()
}

fn part2(rucksacks: &[Rucksack]) -> usize {
    let mut sum = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        let group = &rucksacks[i..(i + 3)];

        let shared_items = group
            .iter()
            .map(|r| {
                r.compartments
                    .iter()
                    .map(|comp| comp.iter().map(|i| *i))
                    .flatten()
            })
            .map(|items| items.collect::<HashSet<Item>>())
            .reduce(|accum, item| &accum & &item)
            .unwrap_or_default();

        sum += shared_items.iter().map(|i| i.priority()).sum::<usize>();
    }

    sum
}

fn main() -> Result<()> {
    let rucksacks = parse("data/day03/input", 2)?;

    println!("Part 1: {}", part1(&rucksacks[..]));
    println!("Part 2: {}", part2(&rucksacks[..]));

    Ok(())
}
