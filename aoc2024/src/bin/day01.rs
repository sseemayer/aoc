use std::collections::HashMap;

use anyhow::Result;

fn part1(numbers_a: &[usize], numbers_b: &[usize]) -> Result<()> {
    let mut numbers_a = numbers_a.to_vec();
    let mut numbers_b = numbers_b.to_vec();

    numbers_a.sort();
    numbers_b.sort();

    let sum: usize = numbers_a
        .iter()
        .zip(numbers_b.iter())
        .map(|(a, b)| usize::abs_diff(*a, *b))
        .sum();

    println!("Part 1: {}", sum);

    Ok(())
}

fn part2(numbers_a: &[usize], numbers_b: &[usize]) -> Result<()> {
    let mut b_counts: HashMap<usize, usize> = HashMap::new();
    for b in numbers_b {
        *b_counts.entry(*b).or_default() += 1;
    }

    let sum: usize = numbers_a
        .iter()
        .map(|a| b_counts.get(a).unwrap_or(&0) * a)
        .sum();

    println!("Part 2: {}", sum);
    Ok(())
}

fn main() -> Result<()> {
    let mut numbers_a: Vec<usize> = Vec::new();
    let mut numbers_b: Vec<usize> = Vec::new();
    for line in aoc::io::read_lines::<String, _>((2024, 1))? {
        let Some((a, b)) = line.split_once(" ") else {
            continue;
        };

        let Ok(a) = a.trim().parse() else { continue };
        let Ok(b) = b.trim().parse() else { continue };

        numbers_a.push(a);
        numbers_b.push(b);
    }

    part1(&numbers_a[..], &numbers_b[..])?;
    part2(&numbers_a[..], &numbers_b[..])?;

    Ok(())
}
