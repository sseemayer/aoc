use std::fs::File;

use aoc::map::{MapError, ParseMapTile};
use colored::Colorize;
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

    #[snafu(display("Map reading error: {}", source))]
    ReadMap { source: MapError },
}

type Map = aoc::map::Map<[i16; 2], Tile>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    Corner,
    Letter(char),
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '|' => Some(Tile::Vertical),
            '-' => Some(Tile::Horizontal),
            '+' => Some(Tile::Corner),
            ' ' => None,
            _ => Some(Tile::Letter(c)),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Vertical => "┃".white(),
                Tile::Horizontal => "━".white(),
                Tile::Corner => "╋".yellow(),
                Tile::Letter(a) => format!("{}", a).green(),
            }
        )
    }
}

fn walk(map: &Map) -> (Vec<char>, usize) {
    let [mut i, mut j] = map
        .find_all(&Tile::Vertical)
        .iter()
        .min_by_key(|[i, _j]| *i)
        .unwrap();

    let mut direction = Direction::Down;
    let mut seen = Vec::new();
    let mut steps = 0;

    loop {
        steps += 1;
        match direction {
            Direction::Up => i -= 1,
            Direction::Right => j += 1,
            Direction::Down => i += 1,
            Direction::Left => j -= 1,
        }

        match map.get(&[i, j]) {
            Some(Tile::Vertical) | Some(Tile::Horizontal) => {}
            Some(Tile::Corner) => {
                let up = map.get(&[i - 1, j]);
                let down = map.get(&[i + 1, j]);
                let left = map.get(&[i, j - 1]);
                let right = map.get(&[i, j + 1]);

                direction = match (&direction, up, right, down, left) {
                    (Direction::Up, None, Some(_), Some(_), None) => Direction::Right,
                    (Direction::Up, None, None, Some(_), Some(_)) => Direction::Left,

                    (Direction::Right, Some(_), None, None, Some(_)) => Direction::Up,
                    (Direction::Right, None, None, Some(_), Some(_)) => Direction::Down,

                    (Direction::Down, Some(_), Some(_), None, None) => Direction::Right,
                    (Direction::Down, Some(_), None, None, Some(_)) => Direction::Left,

                    (Direction::Left, Some(_), Some(_), None, None) => Direction::Up,
                    (Direction::Left, None, Some(_), Some(_), None) => Direction::Down,

                    _ => panic!(
                        "Bad corner: {:?} {:?} {:?} {:?} {:?}",
                        direction, up, right, down, left
                    ),
                };
            }
            Some(Tile::Letter(a)) => {
                seen.push(*a);
            }
            None => {
                break;
            }
        }
    }

    (seen, steps)
}

fn main() -> Result<()> {
    let map = Map::read(&mut File::open("data/day19/input").context(Io)?).context(ReadMap)?;

    let (path, steps) = walk(&map);

    println!("Part 1: {}", path.into_iter().collect::<String>());
    println!("Part 2: {}", steps);

    Ok(())
}
