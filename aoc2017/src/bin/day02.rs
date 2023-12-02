use anyhow::{Context, Result};

fn main() -> Result<()> {
    let data: Vec<Vec<i64>> = std::fs::read_to_string("data/day02/input")?
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse().context("Parse input int"))
                .collect::<Result<_>>()
        })
        .collect::<Result<_>>()?;

    let mut checksum = 0;
    let mut ressum = 0;
    for row in data.iter() {
        let min = row.iter().min().unwrap();
        let max = row.iter().max().unwrap();

        checksum += max - min;

        for (i, a) in row.iter().enumerate() {
            for (j, b) in row.iter().enumerate() {
                if i == j {
                    continue;
                }
                if a % b == 0 {
                    ressum += a / b;
                }
            }
        }
    }

    println!("Part 1: {}", checksum);
    println!("Part 2: {}", ressum);

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
