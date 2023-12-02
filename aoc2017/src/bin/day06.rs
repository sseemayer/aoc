use std::collections::HashMap;

use anyhow::{Context, Result};

fn step(banks: &mut [u16]) {
    let (pos, n) = banks
        .iter()
        .enumerate()
        .max_by_key(|(i, b)| (*b, banks.len() - i))
        .unwrap();

    let mut pos = pos;
    let mut n = *n;
    banks[pos] = 0;
    while n > 0 {
        pos = (pos + 1) % banks.len();
        banks[pos] += 1;
        n -= 1;
    }
}

fn main() -> Result<()> {
    let banks: Vec<u16> = std::fs::read_to_string("data/day06/input")?
        .trim()
        .split_whitespace()
        .map(|t| t.parse().context("Parse input int"))
        .collect::<Result<_>>()?;

    // let banks = vec![0, 2, 7, 0];
    println!("banks: {:?}", banks);

    let mut seen: HashMap<Vec<u16>, usize> = HashMap::new();
    let mut state = banks.clone();
    let mut steps = 0;
    while !seen.contains_key(&state) {
        seen.insert(state.clone(), steps);
        steps += 1;
        step(&mut state);
        println!("{} {:?}", steps, state);
    }

    println!("Loop {} steps", steps - seen[&state]);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
