use std::collections::HashMap;

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

fn main() -> Result<()> {
    let data: Vec<Vec<char>> = std::fs::read_to_string("data/day06/input")
        .context(Io)?
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let columns = data[0].len();
    let mut out1 = Vec::new();
    let mut out2 = Vec::new();
    for i in 0..columns {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for row in &data {
            let c = row[i];
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut counts: Vec<(usize, char)> = counts.into_iter().map(|(c, n)| (n, c)).collect();
        counts.sort();
        out1.push(counts.last().unwrap().1);
        out2.push(counts.first().unwrap().1);
    }

    println!("Part 1: {}", out1.into_iter().collect::<String>());
    println!("Part 2: {}", out2.into_iter().collect::<String>());

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
