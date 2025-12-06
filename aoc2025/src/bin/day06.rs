use std::str::FromStr;

use anyhow::{Context, Error, Result, bail};
use colored::Colorize;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim() {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => bail!("invalid operator: {}", s),
        }
    }
}

impl Operator {
    fn apply(&self, nums: &[usize]) -> usize {
        match self {
            Operator::Add => nums.iter().sum(),
            Operator::Multiply => nums.iter().product(),
        }
    }
}

// From the tabular number data, extract the numbers interpreting them as one number per row.
fn get_nums_rowwise(numbers: &Vec<&[char]>) -> Result<Vec<usize>> {
    let nums = numbers
        .iter()
        .map(|n| {
            n.iter()
                .collect::<String>()
                .trim()
                .parse::<usize>()
                .context("parsing number")
        })
        .collect::<Result<Vec<usize>>>()?;
    Ok(nums)
}

/// From the tabular number data, extract the numbers interpreting them as one number per column.
fn get_nums_colwise(numbers: &Vec<&[char]>) -> Result<Vec<usize>> {
    let mut nums = Vec::new();
    let len = numbers[0].len();
    for i in 0..len {
        let num_str: String = numbers.iter().map(|n| n[i]).collect();
        let num = num_str.trim().parse::<usize>().context("parsing number")?;
        nums.push(num);
    }
    Ok(nums)
}

/// Parse the input into a vector of Problem structs.
fn process(input: &str) -> Result<(usize, usize)> {
    // 123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   +

    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut last_column = 0;

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for i in 0..lines[0].len() + 1 {
        if i == lines[0].len() || lines.iter().all(|line| line[i].is_whitespace()) {
            let operator = Operator::from_str(
                &lines.last().unwrap()[last_column..i]
                    .iter()
                    .collect::<String>(),
            )?;

            let numbers: Vec<&[char]> = lines[..lines.len() - 1]
                .iter()
                .map(|line| &line[last_column..i])
                .collect();

            let nums_part1 = get_nums_rowwise(&numbers)?;
            let nums_part2 = get_nums_colwise(&numbers)?;

            part1_sum += operator.apply(&nums_part1);
            part2_sum += operator.apply(&nums_part2);

            last_column = i + 1;
        }
    }

    Ok((part1_sum, part2_sum))
}

fn main() -> Result<()> {
    //let (part1_sum, part2_sum) = process(&aoc::io::read_all("data/day06/example")?)?;
    let (part1_sum, part2_sum) = process(&aoc::io::read_all((2025, 6))?)?;
    println!("{} {}", "Part 1:".bold().green(), part1_sum);
    println!("{} {}", "Part 1:".bold().green(), part2_sum);
    Ok(())
}
