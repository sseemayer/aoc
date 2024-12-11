use std::collections::HashMap;

use anyhow::{Error, Result};

#[derive(Debug, Clone)]
struct Rocks {
    counts: HashMap<usize, usize>,
}

impl Rocks {
    fn blink(self) -> Self {
        let mut counts: HashMap<usize, usize> = HashMap::new();

        for (n, count) in self.counts {
            let n_digits = if n == 0 { 1 } else { n.ilog10() + 1 };

            if n == 0 {
                // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
                *counts.entry(1).or_default() += count;
            } else if n_digits % 2 == 0 {
                // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
                // The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved
                // on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)

                let mask = 10usize.pow(n_digits / 2);
                let left = n / mask;
                let right = n % mask;

                *counts.entry(left).or_default() += count;
                *counts.entry(right).or_default() += count;
            } else {
                // If none of the other rules apply, the stone is replaced by a new stone;
                // the old stone's number multiplied by 2024 is engraved on the new stone.

                *counts.entry(n * 2024).or_default() += count;
            }
        }

        Self { counts }
    }
}

impl std::str::FromStr for Rocks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut counts = HashMap::new();
        for n in s.split_whitespace() {
            let n = n.parse()?;
            *counts.entry(n).or_default() += 1;
        }

        Ok(Self { counts })
    }
}

fn main() -> Result<()> {
    let mut rocks: Rocks = aoc::io::read_all((2024, 11))?.parse()?;

    for _ in 0..25 {
        rocks = rocks.blink();
    }

    println!("Part 1: {}", rocks.counts.values().cloned().sum::<usize>());

    for _ in 0..50 {
        rocks = rocks.blink();
    }

    println!("Part 2: {}", rocks.counts.values().cloned().sum::<usize>());

    Ok(())
}
