use anyhow::{anyhow, Result};
use itertools::Itertools;

fn get_calibration_value(line: &str) -> Result<u32> {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    let a = digits
        .first()
        .ok_or(anyhow!("Cannot get first digit for {}", line))?;
    let b = digits
        .last()
        .ok_or(anyhow!("Cannot get last digit for {}", line))?;

    Ok(a * 10 + b)
}

const NUMBER_WORDS: [(&'static str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_calibration_value_words(line: &str) -> Result<u32> {
    let chars = line.chars().collect_vec();

    let mut first_digit = None;
    'outer: for i in 0..chars.len() {
        let view: String = chars[i..].iter().collect();

        for (word, n) in NUMBER_WORDS {
            if view.starts_with(word) || view.starts_with(char::from_digit(n, 10).expect("digit")) {
                first_digit = Some((n, i));
                break 'outer;
            }
        }
    }

    let (a, i) = first_digit.ok_or(anyhow!("No digit or digit word found for {}", line))?;

    let mut last_digit = None;
    'outer: for j in (i..chars.len()).rev() {
        let view: String = chars[j..].iter().collect();
        for (word, n) in NUMBER_WORDS {
            if view.starts_with(word) || view.starts_with(char::from_digit(n, 10).expect("digit")) {
                last_digit = Some((n, i));
                break 'outer;
            }
        }
    }

    let (b, _j) = last_digit.ok_or(anyhow!("No last digit or digit word found for {}", line))?;

    Ok(a * 10 + b)
}

fn main() -> Result<()> {
    let lines: Vec<String> = aoc::io::read_lines("data/day01/input")?;

    let values: Vec<u32> = lines
        .iter()
        .map(|l| get_calibration_value(l))
        .collect::<Result<Vec<u32>>>()?;

    let sum1 = values.iter().sum::<u32>();

    println!("Part 1: {}", sum1);

    let values2 = lines
        .iter()
        .map(|l| get_calibration_value_words(l))
        .collect::<Result<Vec<u32>>>()?;

    let sum2 = values2.iter().sum::<u32>();

    println!("Part 2: {}", sum2);

    Ok(())
}
