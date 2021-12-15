use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use snafu::{ResultExt, Snafu};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Bad line: {}", line))]
    BadLine { line: String },

    #[snafu(display("Number format: {}", source))]
    Num { source: std::num::ParseIntError },
}

#[derive(Debug)]
struct Layer {
    depth: usize,
    range: usize,
}

impl std::str::FromStr for Layer {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (depth, range) = s.split_once(": ").ok_or(Error::BadLine {
            line: s.to_string(),
        })?;

        let depth = depth.parse().context(Num)?;
        let range = range.parse().context(Num)?;

        Ok(Layer { depth, range })
    }
}

fn calculate_severity(layers: &[Layer], delay: usize) -> usize {
    let mut out = 0;
    for layer in layers {
        if (layer.depth + delay) % (layer.range * 2 - 2) == 0 {
            // collision
            out += layer.depth * layer.range;
        }
    }

    out
}

fn caught(layers: &[Layer], delay: usize) -> bool {
    layers
        .iter()
        .any(|l| (l.depth + delay) % (l.range * 2 - 2) == 0)
}

fn main() -> Result<()> {
    let layers = BufReader::new(File::open("data/day13/input").context(Io)?)
        .lines()
        .map(|l| l.context(Io)?.parse())
        .collect::<Result<Vec<Layer>>>()?;

    println!("Part 1: {}", calculate_severity(&layers[..], 0));

    let mut delay = 0;
    while caught(&layers[..], delay) {
        delay += 1;
    }

    println!("Part 2: {}", delay);

    Ok(())
}
