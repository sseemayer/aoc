use std::{num::ParseIntError, str::FromStr};

use anyhow::{anyhow, Error, Result};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_CARD: Regex =
        Regex::new(r"^Card\s+(\d+):([ 0-9]+)\|([ 0-9]+)\s*$").expect("Valid regex");
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    have_numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let m = RE_CARD
            .captures(s)
            .ok_or_else(|| anyhow!("Bad card: '{}'", s))?;

        // let card_number = m.get(1).expect("Always have group 1").as_str().parse()?;

        let winning_numbers = m
            .get(2)
            .expect("Always have group 2")
            .as_str()
            .trim()
            .split_whitespace()
            .map(|n| {
                n.parse()
                    .map_err(|e: ParseIntError| Error::from(e).context("Parse winning number"))
            })
            .collect::<Result<Vec<_>>>()?;
        let have_numbers = m
            .get(3)
            .expect("Always have group 3")
            .as_str()
            .trim()
            .split_whitespace()
            .map(|n| {
                n.parse()
                    .map_err(|e: ParseIntError| Error::from(e).context("Parse have number"))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            winning_numbers,
            have_numbers,
        })
    }
}

impl Card {
    fn get_winning(&self) -> Vec<u32> {
        self.have_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .cloned()
            .collect()
    }

    fn get_points(&self) -> usize {
        let w = self.get_winning();
        let n = w.len() as u32;

        if n > 0 {
            1 << (n - 1)
        } else {
            0
        }
    }
}

fn part2_winnings(cards: &Vec<Card>) -> usize {
    let mut card_counts: Vec<usize> = cards.iter().map(|_c| 1).collect();

    for i in 0..cards.len() {
        let current_count = card_counts[i];
        let current_card = &cards[i];
        let wins = current_card.get_winning().len();

        for j in (i + 1)..=(i + wins) {
            if j >= cards.len() {
                break;
            }
            card_counts[j] += current_count;
        }
    }

    card_counts.iter().sum()
}

fn main() -> Result<()> {
    let cards: Vec<Card> = aoc::io::read_lines("data/day04/input")?;

    let score: usize = cards.iter().map(|c| c.get_points()).sum();

    println!("Part 1: {}", score);

    let winnings = part2_winnings(&cards);

    println!("Part 2: {}", winnings);

    Ok(())
}
