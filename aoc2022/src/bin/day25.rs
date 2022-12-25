use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Sum,
};

use anyhow::{anyhow, bail, Context, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Snafu(usize);

impl std::str::FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n = 0;
        for c in s.trim().chars() {
            n *= 5;
            match c {
                '0' => {}
                '1' => n += 1,
                '2' => n += 2,
                '-' => n -= 1,
                '=' => n -= 2,
                _ => bail!("Bad digit '{}' when parsing '{}'", c, s),
            }
        }

        Ok(Snafu(n))
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        let mut out = 0;
        for n in iter {
            out += n.0;
        }

        Self(out)
    }
}

fn parse(path: &str) -> Result<Vec<Snafu>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| line?.trim().parse())
        .collect()
}

fn main() -> Result<()> {
    let numbers = parse("data/day25/input")?;
    let sum: Snafu = numbers.into_iter().sum();
    println!("Part 1: {}", sum.to_str());
    Ok(())
}

impl Snafu {
    fn to_str(&self) -> String {
        let mut out = String::new();
        let mut n = self.0;
        let mut remainder = 0;
        while n > 0 || remainder > 0 {
            let (digit, new_remainder) = match n % 5 + remainder {
                0 => ('0', 0),
                1 => ('1', 0),
                2 => ('2', 0),
                3 => ('=', 1),
                4 => ('-', 1),
                5 => ('0', 1),
                _ => panic!("this should not happen"),
            };

            out.insert(0, digit);
            remainder = new_remainder;
            n /= 5;
        }

        out
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const PAIRS: [(usize, &'static str); 28] = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
        (1747, "1=-0-2"),
        (906, "12111"),
        (198, "2=0="),
        (11, "21"),
        (201, "2=01"),
        (31, "111"),
        (1257, "20012"),
        (32, "112"),
        (353, "1=-1="),
        (107, "1-12"),
        (7, "12"),
        (3, "1="),
        (37, "122"),
    ];

    #[test]
    fn test_from_str() -> Result<()> {
        for (n, snafu) in PAIRS {
            assert_eq!(Snafu(n), snafu.parse()?);
        }

        Ok(())
    }

    #[test]
    fn test_to_str() {
        for (n, snafu) in PAIRS {
            assert_eq!(Snafu(n).to_str(), snafu);
        }
    }
}
