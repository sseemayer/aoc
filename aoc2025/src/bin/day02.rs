use std::{ops::RangeInclusive, str::FromStr, usize};

use anyhow::{Context, Error, Result, bail};
use colored::Colorize;

/// Count the number of digits in a number.
fn count_digits(n: usize) -> usize {
    n.ilog10() as usize + 1
}

/// Split a number into a given number of blocks of equal size.
///
/// The number is handled as if it was a string, e.g. 123456 with n_blocks 2 -> [123, 456].
fn split_digits(mut n: usize, n_blocks: usize) -> Option<Vec<usize>> {
    let n_digits = count_digits(n);

    if n_digits % n_blocks != 0 {
        return None;
    }

    let block_size = n_digits / n_blocks;
    let divisor = 10_usize.pow(block_size as u32);

    let mut out = Vec::new();
    for _ in 0..n_blocks {
        out.push(n % divisor);
        n /= divisor;
    }
    out.reverse();
    Some(out)
}

/// An ID is valid if it is not made of only repeating blocks of digits.
fn is_valid(n: usize, n_blocks: usize) -> bool {
    let Some(blocks) = split_digits(n, n_blocks) else {
        return true;
    };

    for i in 1..n_blocks {
        if blocks[i] != blocks[0] {
            return true;
        }
    }

    false
}

/// A range of numbers which can be parsed from a string like "123-234"
#[derive(Debug, Clone)]
struct Range(RangeInclusive<usize>);

impl Range {
    fn sum_invalid_ids(&self, max_n_blocks: usize) -> usize {
        self.0
            .clone()
            .filter(|&n| {
                (2..=std::cmp::min(count_digits(n) / 2, max_n_blocks))
                    .any(|n_blocks| !is_valid(n, n_blocks))
            })
            .sum()
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts
            .next()
            .context("missing start of range")?
            .parse::<usize>()
            .with_context(|| format!("invalid start of range parsing {}", s))?;
        let end = parts
            .next()
            .context("missing end of range")?
            .parse::<usize>()
            .with_context(|| format!("invalid end of range parsing {}", s))?;
        if parts.next().is_some() {
            bail!("too many parts in range");
        }
        Ok(Range(start..=end))
    }
}

fn main() -> Result<()> {
    let data: Vec<Range> = aoc::io::read_all((2025, 2))?
        .split(",")
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<Range>>>()?;

    let mut sum1 = 0;
    let mut sum2 = 0;
    for range in data {
        sum1 += range.sum_invalid_ids(2);
        sum2 += range.sum_invalid_ids(usize::MAX);
    }

    println!("{} {}", "Part 1".bold().green(), sum1);
    println!("{} {}", "Part 2".bold().green(), sum2);

    Ok(())
}
