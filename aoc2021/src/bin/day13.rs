use aoc::map::Map;
use aoc2021::io::{read_lines, ReadLinesError};
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

#[derive(Debug)]
enum Command {
    Pixel { x: i64, y: i64 },
    NoOp,
    FoldX { x: i64 },
    FoldY { y: i64 },
}

#[derive(Error, Debug)]
enum ParseCommandError {
    #[error("Bad line: '{}'", .0)]
    BadLine(String),

    #[error(transparent)]
    Int(#[from] std::num::ParseIntError),
}

#[derive(Clone, PartialEq, Eq)]
struct Tile(bool);

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { 'X' } else { ' ' })
    }
}

impl std::str::FromStr for Command {
    type Err = ParseCommandError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("fold along y=") {
            let y = s.split_once("=").unwrap().1.parse()?;
            Ok(Command::FoldY { y })
        } else if s.starts_with("fold along x=") {
            let x = s.split_once("=").unwrap().1.parse()?;
            Ok(Command::FoldX { x })
        } else if s.trim().is_empty() {
            Ok(Command::NoOp)
        } else {
            let (x, y) = s
                .split_once(",")
                .ok_or(ParseCommandError::BadLine(s.to_string()))?;

            let x = x.parse()?;
            let y = y.parse()?;

            Ok(Command::Pixel { x, y })
        }
    }
}

impl Command {
    fn step(&self, mut map: Map<[i64; 2], Tile>) -> Map<[i64; 2], Tile> {
        match self {
            Command::Pixel { x, y } => {
                map.set([*y, *x], Tile(true));
                map
            }

            Command::NoOp => map,
            Command::FoldX { x: fold_x } => {
                let mut new_map = Map::new();
                for ([y, x], tile) in map.data {
                    //        v
                    //   01234567
                    //      76

                    let x = if x > *fold_x { fold_x + fold_x - x } else { x };
                    new_map.set([y, x], tile);
                }
                new_map
            }
            Command::FoldY { y: fold_y } => {
                let mut new_map = Map::new();
                for ([y, x], tile) in map.data {
                    let y = if y > *fold_y { fold_y + fold_y - y } else { y };
                    new_map.set([y, x], tile);
                }
                new_map
            }
        }
    }
}

fn main() -> Result<(), ReadLinesError<Command>> {
    let lines = read_lines("data/day13/input")?;

    let (pixels, commands) = lines
        .into_iter()
        .partition::<Vec<Command>, _>(|c| matches!(c, Command::Pixel { .. }));

    let mut map: Map<[i64; 2], Tile> = Map::new();

    for pix in pixels {
        map = pix.step(map);
    }

    let map_after_one_fold = commands[0].step(map.clone());
    let part1 = map_after_one_fold.find_all(&Tile(true)).len();

    println!("Part 1: {}", part1);

    for cmd in commands {
        map = cmd.step(map);
    }

    println!("{}", map);

    Ok(())
}
