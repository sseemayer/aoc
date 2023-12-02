use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_GAME: Regex = Regex::new(r"^Game (\d+):\s+(.+)\s*$").expect("valid regex");
    static ref RE_CUBE_NUMBER: Regex = Regex::new(r"(\d+)\s+(\w+)").expect("valid regex");
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    cubes: HashMap<String, usize>,
}

impl std::str::FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let m = RE_GAME
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid game: '{}'", s))?;

        let id: usize = m
            .get(1)
            .expect("Capture group for ID always there")
            .as_str()
            .parse()
            .context("Parse game ID")?;

        let rounds: Vec<Round> = m
            .get(2)
            .expect("Capture group for rounds always there")
            .as_str()
            .split(";")
            .map(|r| r.parse())
            .collect::<Result<Vec<Round>>>()?;

        Ok(Self { id, rounds })
    }
}

impl std::str::FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let cubes = s
            .split(",")
            .map(|c| {
                let m = RE_CUBE_NUMBER
                    .captures(c.trim())
                    .ok_or_else(|| anyhow!("Bad cubes pattern: '{}'", c))?;

                let count = m
                    .get(1)
                    .expect("Count group alwatys captures")
                    .as_str()
                    .parse()
                    .context("Parse cube count")?;
                let color = m
                    .get(2)
                    .expect("Color group always captures")
                    .as_str()
                    .to_string();

                Ok((color, count))
            })
            .collect::<Result<HashMap<String, usize>>>()?;

        Ok(Self { cubes })
    }
}

impl Game {
    fn valid_for_bag(&self, bag: &HashMap<String, usize>) -> bool {
        self.rounds.iter().all(|r| r.valid_for_bag(bag))
    }

    fn min_cubes(&self) -> HashMap<String, usize> {
        let mut out = HashMap::new();

        for round in &self.rounds {
            for (color, count) in &round.cubes {
                let n = out.entry(color.to_owned()).or_default();
                *n = usize::max(*n, *count);
            }
        }

        out
    }

    fn power_level(&self) -> usize {
        self.min_cubes().values().product()
    }
}

impl Round {
    fn valid_for_bag(&self, bag: &HashMap<String, usize>) -> bool {
        for (color, count) in &self.cubes {
            let bag_count = bag.get(color).unwrap_or(&0);

            if count > bag_count {
                return false;
            }
        }
        true
    }
}

fn main() -> Result<()> {
    let games: Vec<Game> = aoc::io::read_lines("data/day02/input")?;

    let the_bag = vec![
        ("red".into(), 12),
        ("green".into(), 13),
        ("blue".into(), 14),
    ]
    .into_iter()
    .collect::<HashMap<String, usize>>();

    let sum_ids: usize = games
        .iter()
        .filter_map(|g| {
            if g.valid_for_bag(&the_bag) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {}", sum_ids);

    let sum_levels: usize = games.iter().map(|g| g.power_level()).sum();

    println!("Part 2: {}", sum_levels);

    Ok(())
}
