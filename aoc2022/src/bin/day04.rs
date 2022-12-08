use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use thiserror::Error;

#[derive(Debug)]
struct Range {
    start: usize,
    stop: usize,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.stop >= other.stop
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.start >= other.start && self.start <= other.stop)
            || (self.stop >= other.start && self.stop <= other.stop)
            || self.contains(other)
            || other.contains(self)
    }
}

#[derive(Debug, Error)]
enum ParseRangeError {
    #[error("Bad delimiter: '{}'", _0)]
    BadDelimiter(String),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl std::str::FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, stop) = s
            .split_once("-")
            .ok_or_else(|| ParseRangeError::BadDelimiter(s.to_string()))?;

        let start: usize = start.parse()?;
        let stop: usize = stop.parse()?;

        Ok(Self { start, stop })
    }
}

fn parse(path: &str) -> Result<Vec<(Range, Range)>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| {
            let line = line?;

            let (a, b) = line
                .split_once(",")
                .ok_or_else(|| anyhow!("Expected two ranges per line"))?;

            let a: Range = a.parse()?;
            let b: Range = b.parse()?;

            Ok((a, b))
        })
        .collect()
}

fn count_containing(ranges: &Vec<(Range, Range)>) -> usize {
    ranges
        .into_iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count()
}

fn count_overlapping(ranges: &Vec<(Range, Range)>) -> usize {
    ranges.into_iter().filter(|(a, b)| a.overlaps(b)).count()
}

fn main() -> Result<()> {
    let ranges = parse("data/day04/input")?;

    println!("Part 1: {}", count_containing(&ranges));
    println!("Part 2: {}", count_overlapping(&ranges));

    Ok(())
}
