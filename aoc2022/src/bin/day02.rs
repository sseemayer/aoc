use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, bail, Result};

#[derive(Debug)]
struct Games<T> {
    games: Vec<(T, T)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(Rps::Rock),
            "B" => Ok(Rps::Paper),
            "C" => Ok(Rps::Scissors),
            "X" => Ok(Rps::Rock),
            "Y" => Ok(Rps::Paper),
            "Z" => Ok(Rps::Scissors),
            _ => bail!("Bad token: {}", s),
        }
    }

    fn what_beats(&self) -> Self {
        match self {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
            Rps::Scissors => Rps::Rock,
        }
    }

    fn what_loses(&self) -> Self {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }

    fn score_choice(&self) -> usize {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    fn score(&self, player: &Rps) -> usize {
        if self == player {
            3
        } else if &self.what_beats() == player {
            6
        } else {
            0
        }
    }
}

impl Games<String> {
    fn parse(path: &str) -> Result<Games<String>> {
        let mut games = Vec::new();
        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;
            let (a, b) = line
                .trim()
                .split_once(" ")
                .ok_or_else(|| anyhow!("Cannot split line '{}'", line))?;

            games.push((a.to_string(), b.to_string()));
        }

        Ok(Games { games })
    }

    fn infer_part1(&self) -> Result<Games<Rps>> {
        let games = self
            .games
            .iter()
            .map(|(a, b)| {
                let ra = Rps::from_str(a)?;
                let rb = Rps::from_str(b)?;

                Ok((ra, rb))
            })
            .collect::<Result<Vec<(Rps, Rps)>>>()?;

        Ok(Games { games })
    }

    fn infer_part2(&self) -> Result<Games<Rps>> {
        let games = self
            .games
            .iter()
            .map(|(a, b)| {
                let ra = Rps::from_str(a)?;
                let rb = match &b[..] {
                    "X" => ra.what_loses(),
                    "Y" => ra.clone(),
                    "Z" => ra.what_beats(),
                    _ => bail!("Bad token {}", b),
                };

                Ok((ra, rb))
            })
            .collect::<Result<Vec<(Rps, Rps)>>>()?;

        Ok(Games { games })
    }
}

impl Games<Rps> {
    fn score(&self) -> usize {
        self.games
            .iter()
            .map(|(opponent, player)| player.score_choice() + opponent.score(player))
            .sum()
    }
}

fn main() -> Result<()> {
    let games = Games::parse("data/day02/input")?;

    println!("Part 1: {}", games.infer_part1()?.score());
    println!("Part 1: {}", games.infer_part2()?.score());

    Ok(())
}
