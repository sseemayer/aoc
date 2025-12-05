use std::str::FromStr;

use anyhow::{Context, Error, Result, bail};
use colored::Colorize;

/// A range of usize values, inclusive start, inclusive end.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range(usize, usize);

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            bail!("Invalid range format: {}", s);
        }

        let start = parts[0]
            .parse::<usize>()
            .with_context(|| format!("Failed to parse start of range: {}", parts[0]))?;
        let end = parts[1]
            .parse::<usize>()
            .with_context(|| format!("Failed to parse end of range: {}", parts[1]))?;

        if start > end {
            bail!("Start of range must be less than or equal to end: {}", s);
        }

        Ok(Range(start, end))
    }
}

impl Range {
    /// Check if a value is within the range.
    fn contains(&self, value: usize) -> bool {
        value >= self.0 && value <= self.1
    }

    /// Merge overlapping or contiguous ranges.
    fn merge(ranges: &[Range]) -> Vec<Range> {
        if ranges.is_empty() {
            return Vec::new();
        }

        let mut sorted_ranges = ranges.to_vec();
        sorted_ranges.sort_by_key(|r| r.0);

        let mut merged = Vec::new();
        let mut current = sorted_ranges[0];

        for &range in &sorted_ranges[1..] {
            if current.contains(range.0) {
                current.1 = current.1.max(range.1);
            } else {
                merged.push(current);
                current = range;
            }
        }
        merged.push(current);

        merged
    }

    /// Get the size of the range, i.e., the number of values it contains.
    fn size(&self) -> usize {
        self.1 - self.0 + 1
    }
}

fn main() -> Result<()> {
    let input = (2025, 5);
    //let input = "data/day05/example";

    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    for line in aoc::io::read_lines::<String, _>(input)? {
        if line.is_empty() {
            continue;
        } else if line.contains('-') {
            let range: Range = line.parse()?;
            ranges.push(range);
        } else {
            let id = line
                .parse::<usize>()
                .with_context(|| format!("Failed to parse ID: {}", line))?;
            ids.push(id);
        }
    }

    let mut part1 = 0;
    for id in &ids {
        if ranges.iter().any(|r| r.contains(*id)) {
            part1 += 1;
        }
    }

    println!("{} {}", "Part 1:".bold().green(), part1);

    let merged = Range::merge(&ranges);
    let mut part2 = 0;
    for range in &merged {
        part2 += range.size();
    }

    println!("{} {}", "Part 2:".bold().green(), part2);

    Ok(())
}
