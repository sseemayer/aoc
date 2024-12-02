use std::str::FromStr;

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<i32>,
}

fn is_safe(mut levels: impl Iterator<Item = i32>, tolerance: usize) -> bool {
    let Some(mut a) = levels.next() else {
        return false;
    };

    let mut signum = None;
    let mut count = 0;
    for b in levels {
        let delta = b - a;

        let safe = (1..=3).contains(&delta.abs())
            && match signum {
                Some(sgn) => delta.signum() == sgn,
                None => {
                    signum = Some(delta.signum());
                    true
                }
            };

        if safe {
            a = b;
        } else {
            count += 1;
        }

        if count > tolerance {
            return false;
        }
    }
    true
}

impl Report {
    fn is_safe(&self, tolerance: usize) -> bool {
        is_safe(self.levels.iter().cloned(), tolerance)
            || is_safe(self.levels.iter().rev().cloned(), tolerance)
    }
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let levels = s
            .split_whitespace()
            .map(|n| n.parse().context("Parse level"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Report { levels })
    }
}

fn main() -> Result<()> {
    let reports: Vec<Report> = aoc::io::read_lines((2024, 2))?;

    let part1 = reports.iter().filter(|r| r.is_safe(0)).count();
    println!("Part 1: {}", part1);

    let part2 = reports.iter().filter(|r| r.is_safe(1)).count();
    println!("Part 2: {}", part2);

    Ok(())
}
