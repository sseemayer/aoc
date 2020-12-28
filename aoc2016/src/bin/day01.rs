use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use snafu::{ResultExt, Snafu};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Invalid turn direction: {}", direction))]
    ParseDirection { direction: String },

    #[snafu(display("Invalid number: {}", source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self, turn: &TurnDirection) -> Self {
        match (self, turn) {
            (Direction::North, TurnDirection::Left) | (Direction::South, TurnDirection::Right) => {
                Direction::West
            }
            (Direction::East, TurnDirection::Left) | (Direction::West, TurnDirection::Right) => {
                Direction::North
            }
            (Direction::South, TurnDirection::Left) | (Direction::North, TurnDirection::Right) => {
                Direction::East
            }
            (Direction::West, TurnDirection::Left) | (Direction::East, TurnDirection::Right) => {
                Direction::South
            }
        }
    }

    fn walk(&self, x: &mut i64, y: &mut i64) {
        match self {
            Direction::North => *y -= 1,
            Direction::East => *x += 1,
            Direction::South => *y += 1,
            Direction::West => *x -= 1,
        }
    }
}

#[derive(Debug)]
enum TurnDirection {
    Left,
    Right,
}

impl std::str::FromStr for TurnDirection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "L" => Ok(TurnDirection::Left),
            "R" => Ok(TurnDirection::Right),
            _ => Err(Error::ParseDirection {
                direction: s.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    turn: TurnDirection,
    walk: i64,
}

impl std::str::FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let turn: TurnDirection = s[..1].parse()?;
        let walk: i64 = s[1..].parse().context(ParseInt {
            data: s[1..].to_string(),
        })?;

        Ok(Instruction { turn, walk })
    }
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = std::fs::read_to_string("data/day01/input")
        .context(Io)?
        .trim()
        .split(", ")
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut direction = Direction::North;
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    seen.insert((0, 0));

    for (i, inst) in instructions.iter().enumerate() {
        direction = direction.turn(&inst.turn);

        for _ in 0..inst.walk {
            direction.walk(&mut x, &mut y);
            if seen.contains(&(x, y)) {
                println!(
                    "Revisit {} {} on turn {}, distance = {}",
                    x,
                    y,
                    i,
                    x.abs() + y.abs()
                );
            }

            seen.insert((x, y));
        }
    }
    println!(
        "Final location: {} {}, distance = {}",
        x,
        y,
        x.abs() + y.abs()
    );

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
