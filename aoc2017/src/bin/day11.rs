use snafu::{ResultExt, Snafu};

use colored::Colorize;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },
}

#[derive(Debug)]
enum Direction {
    NorthWest,
    North,
    NorthEast,
    SouthWest,
    South,
    SouthEast,
}

impl Direction {
    fn to_axial(&self) -> (i64, i64) {
        match self {
            Direction::NorthWest => (-1, 0),
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::SouthWest => (-1, 1),
            Direction::South => (0, 1),
            Direction::SouthEast => (1, 0),
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s.to_lowercase().trim() {
            "nw" => Direction::NorthWest,
            "n" => Direction::North,
            "ne" => Direction::NorthEast,
            "sw" => Direction::SouthWest,
            "s" => Direction::South,
            "se" => Direction::SouthEast,
            _ => panic!("Bad direction"),
        })
    }
}

fn axial_distance((aq, ar): (i64, i64), (bq, br): (i64, i64)) -> i64 {
    ((aq - bq).abs() + (aq + ar - bq - br).abs() + (ar - br).abs()) / 2
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/day11/input").context(Io)?;
    let directions = input
        .split(",")
        .map(|d| Ok(d.parse()?))
        .collect::<Result<Vec<Direction>>>()?;

    let mut pos = (0, 0);

    let mut max_dist = 0;
    for dir in &directions {
        let delta = dir.to_axial();
        pos.0 += delta.0;
        pos.1 += delta.1;

        let dist = axial_distance((0, 0), pos);
        max_dist = max_dist.max(dist);
    }

    println!("Part 1: {}", axial_distance((0, 0), pos));
    println!("Part 2: {}", max_dist);

    Ok(())
}
