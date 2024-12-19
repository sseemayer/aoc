use std::collections::HashSet;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone)]
struct Input {
    towels: HashSet<Vec<char>>,
    splits: HashSet<usize>,
    designs: Vec<String>,
}

impl std::str::FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (towels, designs) = s
            .split_once("\n\n")
            .ok_or(anyhow!("expect empty delimiter line"))?;

        let towels: HashSet<Vec<char>> = towels
            .split(",")
            .map(|s| s.trim().chars().collect())
            .collect();
        let splits: HashSet<usize> = towels.iter().map(|s| s.len()).collect();

        let designs = designs.lines().map(|d| d.trim().to_string()).collect();

        Ok(Self {
            towels,
            splits,
            designs,
        })
    }
}

impl Input {
    fn ways_to_build(&self, design: &str) -> usize {
        //println!("Check: {}", design);
        let design: Vec<char> = design.chars().collect();

        // dynamic programming array of how many ways a prefix of length n can be constructed
        let mut ways: Vec<usize> = (0..=design.len()).map(|_| 0).collect();
        ways[0] = 1;

        for solution_length in 1..=design.len() {
            for &split_length in &self.splits {
                // try to find a solution for a prefix of length split_length\
                // that ends after solution_length characters
                if split_length > solution_length {
                    // cannot go before beginning of string
                    continue;
                }

                let check_prefix = &design[(solution_length - split_length)..solution_length];

                if !self.towels.contains(check_prefix) {
                    // prefix is not valid
                    continue;
                }

                ways[solution_length] += ways[solution_length - split_length];
            }

            // println!(
            //     "{:?} {}",
            //     &design[..solution_length],
            //     ways[solution_length]
            // );
        }

        ways[design.len()]
    }
}

fn main() -> Result<()> {
    let input: Input = aoc::io::read_all((2024, 19))?.parse()?;
    //let input: Input = aoc::io::read_all("data/day19/example")?.parse()?;

    //println!("{:?}", input.towels);

    let mut can_build = 0;
    let mut ways_to_build = 0;

    for design in &input.designs {
        let n = input.ways_to_build(design);
        ways_to_build += n;
        if n > 0 {
            can_build += 1;
        }
    }

    println!("Part 1: {}", can_build);
    println!("Part 2: {}", ways_to_build);

    Ok(())
}
