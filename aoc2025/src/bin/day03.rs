use std::str::FromStr;

use anyhow::{Context, Error, Result};
use colored::Colorize;

#[derive(Debug, Clone)]
struct Bank(Vec<u32>);

impl FromStr for Bank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let nums = s
            .chars()
            .map(|c| c.to_digit(10).context("invalid digit"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Bank(nums))
    }
}

impl Bank {
    fn max_joltage(&self, max_digits: usize) -> Result<usize> {
        let mut joltage = 0;

        let mut left = 0;
        let mut right = self.0.len() - max_digits;
        let mut digits = 0;

        while digits < max_digits {
            let (max_pos, &max_digit) = self.0[left..=right]
                .iter()
                .enumerate()
                // we need the leftmost max digit, so we get the max by (n, -i)
                .max_by_key(|&(i, n)| (n, -(i as isize)))
                .context("empty range")?;

            joltage = joltage * 10 + max_digit as usize;
            digits += 1;

            left += max_pos + 1;
            right += 1;
        }

        Ok(joltage)
    }
}

fn sum_joltage(banks: &Vec<Bank>, max_digits: usize) -> Result<usize> {
    Ok(banks
        .iter()
        .map(|bank| bank.max_joltage(max_digits))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<usize>())
}

fn main() -> Result<()> {
    //let banks: Vec<Bank> = aoc::io::read_lines("data/day03/example")?;
    let banks: Vec<Bank> = aoc::io::read_lines((2025, 3))?;

    println!("{} {}", "Part 1:".bold().green(), sum_joltage(&banks, 2)?);
    println!("{} {}", "Part 2:".bold().green(), sum_joltage(&banks, 12)?);

    Ok(())
}
