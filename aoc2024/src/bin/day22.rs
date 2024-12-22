use std::collections::HashMap;

use anyhow::{Context, Error, Result};

const MODULO: usize = 0x1000000;

#[derive(Debug, Clone)]
struct PriceIterator(usize);

impl std::str::FromStr for PriceIterator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.parse().context("parse seed")?))
    }
}

impl Iterator for PriceIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.0;
        let n = ((n << 6) ^ n) % MODULO;
        let n = ((n >> 5) ^ n) % MODULO;
        let n = ((n << 11) ^ n) % MODULO;
        self.0 = n;
        Some(n)
    }
}

impl PriceIterator {
    fn window_prices(self, max_iterations: usize) -> HashMap<[i8; 4], usize> {
        let mut delta = [0i8; 4];
        let mut last = self.0;
        let mut counts: HashMap<[i8; 4], usize> = Default::default();

        let iter = self.take(max_iterations);

        for (i, n) in iter.enumerate() {
            delta[0] = delta[1];
            delta[1] = delta[2];
            delta[2] = delta[3];
            delta[3] = ((n % 10) as i8) - ((last % 10) as i8);

            if i >= 4 {
                counts.entry(delta).or_insert(n % 10);
            }

            last = n;
        }

        counts
    }
}

fn main() -> Result<()> {
    let buyers: Vec<PriceIterator> = aoc::io::read_lines((2024, 22))?;
    //let buyers: Vec<PriceIterator> = aoc::io::read_lines("data/day22/example2")?;

    let part1 = buyers
        .clone()
        .into_iter()
        .filter_map(|i| i.take(2000).last())
        .sum::<usize>();

    println!("Part 1: {}", part1);

    let mut window_sums: HashMap<[i8; 4], usize> = HashMap::new();
    for buyer in buyers {
        let window_prices = buyer.window_prices(2000);
        for (k, v) in window_prices {
            *window_sums.entry(k).or_default() += v;
        }
    }

    let (_best_window, best_sum) = window_sums.iter().max_by_key(|(_k, v)| **v).unwrap();

    println!("Part 2: {}", best_sum);

    Ok(())
}
