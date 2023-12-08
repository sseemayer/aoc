use std::str::FromStr;

use anyhow::{anyhow, bail, Context, Result};

#[derive(Debug)]
struct Races {
    races: Vec<Race>,
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut times: Vec<usize> = Vec::new();
        let mut distances: Vec<usize> = Vec::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            } else if line.starts_with("Time:") {
                times.extend(
                    line.trim_start_matches("Time:")
                        .trim()
                        .split_whitespace()
                        .map(|t| t.parse().context("Parsing time token"))
                        .collect::<Result<Vec<usize>>>()?,
                );
            } else if line.starts_with("Distance:") {
                distances.extend(
                    line.trim_start_matches("Distance:")
                        .trim()
                        .split_whitespace()
                        .map(|t| t.parse().context("Parsing distance token"))
                        .collect::<Result<Vec<usize>>>()?,
                );
            } else {
                bail!("Unknown line: '{}'", line);
            }
        }

        if times.len() != distances.len() {
            bail!(
                "Mismatching lengths: times: {:?} distances {:?}",
                times,
                distances
            );
        }

        let races = times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(time, distance)| Race { time, distance })
            .collect();

        Ok(Races { races })
    }
}

impl Races {
    fn multiply_ways_to_win(&self) -> usize {
        self.races.iter().map(|r| r.count_ways_to_win()).product()
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl FromStr for Race {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut time: Option<usize> = None;
        let mut distance: Option<usize> = None;

        for line in s.lines() {
            let line = line.trim().replace(" ", "");
            if line.is_empty() {
                continue;
            } else if line.starts_with("Time:") {
                time = Some(
                    line.trim_start_matches("Time:")
                        .parse()
                        .context("Parse time")?,
                );
            } else if line.starts_with("Distance:") {
                distance = Some(
                    line.trim_start_matches("Distance:")
                        .parse()
                        .context("Parse distance")?,
                );
            } else {
                bail!("Unknown line: '{}'", line);
            }
        }

        let time = time.ok_or_else(|| anyhow!("Did not encounter time"))?;
        let distance = distance.ok_or_else(|| anyhow!("Did not encounter distance"))?;

        Ok(Race { time, distance })
    }
}

impl Race {
    fn simulate(&self, hold_time: usize) -> bool {
        let distance = hold_time * (self.time - hold_time);
        distance > self.distance
    }

    fn count_ways_to_win(&self) -> usize {
        (1..self.time).filter(|t| self.simulate(*t)).count()
    }
}

fn main() -> Result<()> {
    let data = aoc::io::read_all("data/day06/input")?;
    let races: Races = data.parse()?;
    println!("Part 1: {}", races.multiply_ways_to_win());

    let race: Race = data.parse()?;
    println!("Part 2: {}", race.count_ways_to_win());

    Ok(())
}
