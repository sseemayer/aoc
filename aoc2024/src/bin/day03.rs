use anyhow::{bail, Result};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_INSTRUCTION: Regex =
        Regex::new(r"(do(?:n't)?)|(?:(mul)\((\d+),(\d+)\))").expect("valid regex");
}

fn main() -> Result<()> {
    let input = aoc::io::read_all((2024, 03))?;

    let mut sum_prod1 = 0;
    let mut sum_prod2 = 0;
    let mut enabled = true;
    for m in RE_INSTRUCTION.captures_iter(&input) {
        let instr = m
            .get(1)
            .or_else(|| m.get(2))
            .map(|m| m.as_str())
            .unwrap_or_default();

        match instr {
            "don't" => {
                enabled = false;
            }
            "do" => {
                enabled = true;
            }
            "mul" => {
                let a: i32 = m
                    .get(3)
                    .and_then(|n| n.as_str().parse().ok())
                    .unwrap_or_default();

                let b: i32 = m
                    .get(4)
                    .and_then(|n| n.as_str().parse().ok())
                    .unwrap_or_default();

                sum_prod1 += a * b;

                if enabled {
                    sum_prod2 += a * b;
                }
            }
            _ => bail!("Illegal instruction: {}", instr),
        }
    }

    println!("Part 1: {}", sum_prod1);
    println!("Part 2: {}", sum_prod2);

    Ok(())
}
