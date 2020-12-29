use std::collections::HashSet;

use snafu::{ResultExt, Snafu};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },
}

fn has_no_duplicates(words: &[String]) -> bool {
    let set: HashSet<&String> = words.iter().collect();
    set.len() == words.len()
}

fn has_no_anagram(words: &[String]) -> bool {
    let set: HashSet<Vec<char>> = words
        .iter()
        .map(|s| {
            let mut sorted: Vec<char> = s.chars().collect();
            sorted.sort();
            sorted
        })
        .collect();

    set.len() == words.len()
}

fn main() -> Result<()> {
    let mut passphrases: Vec<Vec<String>> = std::fs::read_to_string("data/day04/input")
        .context(Io)?
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    let valid_passphrases: Vec<_> = passphrases
        .iter()
        .filter(|w| has_no_duplicates(w))
        .collect();

    println!("Part 1: {}", valid_passphrases.len());

    let super_valid_passphrases: Vec<_> = valid_passphrases
        .iter()
        .filter(|w| has_no_anagram(w))
        .collect();

    println!("Part 2: {}", super_valid_passphrases.len());

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
