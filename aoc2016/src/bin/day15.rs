use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_DISC: Regex =
        Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).")
            .unwrap();
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[derive(Debug, Clone)]
struct Disc {
    number: i64,
    n_positions: i64,
    offset: i64,
}

impl std::str::FromStr for Disc {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let cap = RE_DISC.captures(s).ok_or(anyhow!("Bad disc: '{}'", s))?;

        let number = cap.get(1).unwrap().as_str();
        let n_positions = cap.get(2).unwrap().as_str();
        let offset = cap.get(3).unwrap().as_str();

        let number: i64 = number.parse().context("Parse disc number")?;
        let n_positions: i64 = n_positions.parse().context("Parse disc position")?;
        let offset: i64 = offset.parse().context("Parse disc offset")?;

        Ok(Disc {
            number,
            n_positions,
            offset,
        })
    }
}

fn solve(discs: &[Disc]) -> Option<i64> {
    let modulii: Vec<i64> = discs.iter().map(|d| d.n_positions).collect();
    let residues: Vec<i64> = discs
        .iter()
        .map(|d| {
            let mut o = -d.number - d.offset;
            while o < 0 {
                o += d.n_positions
            }
            o
        })
        .collect();

    // find t so that for each disc:
    //
    // t + i + o = 0 (mod p)
    //
    // t = -i -o (mod p)
    //
    // i : disc index
    // o : disc offset
    // p : disc number of positions
    chinese_remainder(&residues, &modulii)
}

fn main() -> Result<()> {
    let discs1: Vec<Disc> = aoc::io::read_lines("data/day15/input")?;

    if let Some(t) = solve(&discs1) {
        println!("Part 1: {}", t);
    }

    let mut discs2 = discs1.clone();
    discs2.push(Disc {
        number: discs1.len() as i64 + 1,
        n_positions: 11,
        offset: 0,
    });

    if let Some(t) = solve(&discs2) {
        println!("Part 2: {}", t);
    }

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
