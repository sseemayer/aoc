use std::fs::File;
use std::io::{BufRead, BufReader};

use pest::Parser;

#[macro_use]
extern crate pest_derive;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error on '{}': {}", filename, source))]
    Io {
        filename: String,
        source: std::io::Error,
    },

    #[snafu(display("Number parsing error: {}", source))]
    ParseNumber { source: std::num::ParseIntError },

    #[snafu(display("Policy parsing error: {}", source))]
    ParsePolicy { source: pest::error::Error<Rule> },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[grammar = "day02.pest"]
pub struct PWPolicyParser;

#[derive(Debug)]
struct PasswordWithPolicy {
    letter: char,
    min: usize,
    max: usize,

    password: String,
}

impl PasswordWithPolicy {
    fn is_valid_count(&self) -> bool {
        let count = count_letter(&self.password, self.letter);
        if count < self.min {
            return false;
        } else if count > self.max {
            return false;
        } else {
            return true;
        }
    }

    fn is_valid_position(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        let a = chars[self.min - 1];
        let b = chars[self.max - 1];

        return ((a == self.letter) || (b == self.letter)) && (a != b);
    }
}

fn count_letter(s: &str, c: char) -> usize {
    return s.chars().filter(|a| *a == c).count();
}

impl std::str::FromStr for PasswordWithPolicy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parse = PWPolicyParser::parse(Rule::policy, s)
            .context(ParsePolicy)?
            .next()
            .unwrap();

        let mut inner = parse.into_inner().collect::<Vec<_>>();

        let range = inner.remove(0).into_inner().collect::<Vec<_>>();
        let min: usize = range[0].as_str().parse().context(ParseNumber)?;
        let max: usize = range[1].as_str().parse().context(ParseNumber)?;

        let letter: char = inner[0].as_str().chars().next().unwrap();

        let password = inner[1].as_str().to_string();

        Ok(PasswordWithPolicy {
            letter,
            min,
            max,
            password,
        })
    }
}

fn main() -> Result<()> {
    let filename = "data/day02/input";
    let f = File::open(filename).context(Io {
        filename: filename.to_string(),
    })?;

    let br = BufReader::new(f);

    let pwp: Vec<PasswordWithPolicy> = br
        .lines()
        .map(|l| {
            l.context(Io {
                filename: "data/day01/input".to_owned(),
            })
        })
        .map(|l| l?.parse())
        .collect::<Result<Vec<_>>>()?;

    let num_valid_1 = pwp.iter().filter(|p| p.is_valid_count()).count();
    let num_valid_2 = pwp.iter().filter(|p| p.is_valid_position()).count();
    println!("Part 1: Got {} valid passwords", num_valid_1);
    println!("Part 2: Got {} valid passwords", num_valid_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rules() -> Vec<PasswordWithPolicy> {
        return "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"
            .split("\n")
            .map(|l| l.parse().unwrap())
            .collect();
    }

    #[test]
    fn test_counts() {
        let pwp = make_rules();
        assert_eq!(pwp.len(), 3);

        assert!(pwp[0].is_valid_count());
        assert!(!pwp[1].is_valid_count());
        assert!(pwp[2].is_valid_count());
    }

    #[test]
    fn test_position() {
        let pwp = make_rules();
        assert_eq!(pwp.len(), 3);

        assert!(pwp[0].is_valid_position());
        assert!(!pwp[1].is_valid_position());
        assert!(!pwp[2].is_valid_position());
    }
}
